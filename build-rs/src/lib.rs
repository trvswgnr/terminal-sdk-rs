use quote::{ToTokens, quote};
use std::{collections::BTreeMap, fs, path::Path, str::FromStr};
use syn::{FnArg, ItemFn, Pat, ReturnType, Type, parse_file, punctuated::Punctuated};

/// generic error type for build script operations
pub type BuildError = Box<dyn std::error::Error>;

/// represents a function parameter with its name and type (as strings)
#[derive(Debug)]
struct ParamInfo {
    /// the parameter name
    name: String,
    /// the parameter type
    ty: String,
}

/// holds the success and error types (as strings) extracted from a `Result<T, Error<E>>`
#[derive(Debug)]
struct ResultTypesInfo {
    /// the `T` in `Result<T, Error<E>>`
    value: String,
    /// the `E` in `Result<T, Error<E>>`
    error: String,
}

/// contains all the info needed to generate a client method
/// that correctly wraps an API function while maintaining its original details.
#[derive(Debug)]
pub struct FunctionInfo {
    /// which API module this function belongs to
    module_name: String,
    /// name of the function to call
    function_name: String,
    /// success type (`T` in `Result<T, E>`)
    result_types: ResultTypesInfo,
    /// function parameters (excluding configuration)
    parameters: Vec<ParamInfo>,
    /// function documentation to preserve in client
    documentation: String,
}

/// discovers all API module names by scanning the source directory.
/// this allows the build script to automatically find all API modules
/// without requiring manual registration of new modules.
///
/// specifically excludes:
/// - mod.rs: contains module organization/exports, not API endpoints
/// - configuration.rs: contains shared config types used across modules
/// - Non-Rust files
pub fn discover_api_module_names(folder_path: &Path) -> Result<Vec<String>, BuildError> {
    let names = fs::read_dir(folder_path)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let is_rs_file = path.extension()?.to_str()? == "rs";

            if !is_rs_file {
                return None;
            }

            let file_name = path.file_stem()?.to_string_lossy().into_owned();

            // skip module organization and shared config files since we know they don't contain API endpoints
            (!["mod", "configuration"].contains(&file_name.as_str())).then_some(file_name)
        })
        .collect();

    Ok(names)
}

/// parses API functions from discovered modules and gets the metadata needed
/// for client code generation.
/// reads each module file, finds valid API functions, extracts type information,
/// and returns a sorted list of fn metadata organized by module.
pub fn parse_api_functions(
    folder_path: &Path,
    api_modules: &[String],
) -> Result<Vec<FunctionInfo>, BuildError> {
    let mut functions: Vec<FunctionInfo> = api_modules
        .iter()
        .map(|module_name| -> Result<Vec<FunctionInfo>, BuildError> {
            let module_path = folder_path.join(format!("{}.rs", module_name));
            let source = fs::read_to_string(&module_path)?;

            let syntax = parse_file(&source)?;

            let module_functions: Vec<FunctionInfo> = syntax
                .items
                .into_iter()
                .filter_map(|item| match item {
                    syn::Item::Fn(func) => parse_function(func, module_name),
                    _ => None,
                })
                .collect();

            Ok(module_functions)
        })
        .collect::<Result<Vec<_>, BuildError>>()?
        .into_iter()
        .flatten()
        .collect();

    functions.sort_by(|a, b| {
        a.module_name
            .cmp(&b.module_name)
            .then(a.function_name.cmp(&b.function_name))
    });

    Ok(functions)
}

/// extracts metadata from a function if it meets API requirements:
/// - public visibility for external access
/// - async for non-blocking operation
/// - configuration parameter for API setup
///
/// returns `None` if the function doesn't meet these requirements or
/// if the return type doesn't match the expected `Result<T, Error<E>>` pattern.
fn parse_function(func: ItemFn, module_name: &str) -> Option<FunctionInfo> {
    if !is_valid_api_function(&func) {
        return None;
    }

    let function_name = func.sig.ident.to_string();
    let parameters = extract_parameters(&func.sig.inputs);
    let result_types = extract_result_types(&func.sig.output)?;
    let documentation = parse_enum_doc_comment(&func.attrs);

    Some(FunctionInfo {
        module_name: module_name.to_string(),
        function_name,
        result_types,
        parameters,
        documentation,
    })
}

/// checks if a function is a valid API endpoint:
fn is_valid_api_function(func: &ItemFn) -> bool {
    let is_public = matches!(func.vis, syn::Visibility::Public(_));
    let is_async = func.sig.asyncness.is_some();

    let has_config_param = func.sig.inputs.iter().any(|arg| {
        let FnArg::Typed(pat_type) = arg else {
            return false;
        };
        let Pat::Ident(pat_ident) = &*pat_type.pat else {
            return false;
        };
        pat_ident.ident == "configuration"
    });

    is_public && is_async && has_config_param
}

/// extracts function parameters, filtering out the `configuration` param.
/// config will be provided by the client instance rather than passed in each call
fn extract_parameters(inputs: &Punctuated<FnArg, syn::token::Comma>) -> Vec<ParamInfo> {
    inputs
        .iter()
        .filter_map(|arg| {
            let FnArg::Typed(pat_type) = arg else {
                return None;
            };
            let Pat::Ident(pat_ident) = &*pat_type.pat else {
                return None;
            };

            let param_name = pat_ident.ident.to_string();

            // exclude the configuration parameter bc we use config from the client
            // so we don't need to pass it in every function call
            if param_name == "configuration" {
                return None;
            }

            let param_type = pat_type.ty.to_token_stream().to_string();
            Some(ParamInfo {
                name: param_name,
                ty: param_type,
            })
        })
        .collect()
}

/// extracts the success and error types from a function's return type signature.
/// this is crucial for generating properly typed client methods that maintain
/// the same type safety as the original API functions.
///
/// for example, given a return type of `Result<Vec<String>, Error<CustomError>>`,
/// this will extract:
/// - value type: `Vec<String>`
/// - error type: `CustomError`
///
/// we specifically look for the `Error<E>` pattern in the error position since
/// this is the standard error wrapper used throughout the API.
fn extract_result_types(return_type: &ReturnType) -> Option<ResultTypesInfo> {
    // we only care about functions that explicitly specify a return type
    // functions without return types (`-> ()`) are not valid API endpoints
    let ReturnType::Type(_, ty) = return_type else {
        return None;
    };

    // we expect the return type to be a path (like `Result<...>`) rather than
    // a primitive type or other complex type
    let Type::Path(path) = ty.as_ref() else {
        return None;
    };

    // verify this is actually a `Result` type - we don't support raw returns
    // as all API functions must handle errors explicitly
    let result_segment = path.path.segments.last()?;
    if result_segment.ident != "Result" {
        return None;
    };

    // extract the generic type arguments from `Result<T, E>`
    let generic_args = match &result_segment.arguments {
        syn::PathArguments::AngleBracketed(args) => args,
        _ => return None,
    };

    let args: Vec<_> = generic_args.args.iter().collect();

    // `Result` must have exactly two type parameters:
    // - the success type `T`
    // - the error type `E`
    if args.len() != 2 {
        return None;
    }

    // extract the success type (`T` in `Result<T, E>`)
    let return_type = match args[0] {
        syn::GenericArgument::Type(ty) => ty.to_token_stream().to_string(),
        _ => return None,
    };

    // the error type must be a path (like `Error<CustomError>`)
    let error_path = match args[1] {
        syn::GenericArgument::Type(Type::Path(path)) => path,
        _ => return None,
    };

    // verify the error type uses our `Error` wrapper type
    let error_segment = error_path.path.segments.last()?;
    if error_segment.ident != "Error" {
        return None;
    }

    // extract the custom error type from `Error<CustomError>`
    let error_args = match &error_segment.arguments {
        syn::PathArguments::AngleBracketed(args) => args,
        _ => return None,
    };

    // get the actual error type (`CustomError` from `Error<CustomError>`)
    let error_type = error_args
        .args
        .first()
        .map(|arg| arg.to_token_stream().to_string())?;

    Some(ResultTypesInfo {
        value: return_type,
        error: error_type,
    })
}

/// generates the implementation block containing all API methods for the client.
/// this creates a properly typed wrapper method for each API function that:
/// - maintains the same parameter types
/// - preserves return types
/// - automatically handles configuration
/// - preserves documentation
pub fn generate_api_methods(functions: &[FunctionInfo]) -> proc_macro2::TokenStream {
    let mut all_methods = Vec::new();
    let mut modules: BTreeMap<&str, Vec<&FunctionInfo>> = BTreeMap::new();

    for func in functions {
        modules.entry(&func.module_name).or_default().push(func);
    }

    for (module, funcs) in &modules {
        let module_comment = format!(
            "/// {} API\n///\n",
            module.replace("_api", "").to_uppercase()
        );
        let comment = proc_macro2::TokenStream::from_str(&module_comment).unwrap();
        all_methods.push(comment);

        for func in funcs {
            let params = func
                .parameters
                .iter()
                .map(|param| format!("{}: {}", param.name, param.ty))
                .collect::<Vec<_>>()
                .join(", ");

            let fn_name = syn::Ident::new(&func.function_name, proc_macro2::Span::call_site());
            let module_name = syn::Ident::new(&func.module_name, proc_macro2::Span::call_site());
            let return_type: syn::Type = syn::parse_str(&func.result_types.value).unwrap();
            let error_type: syn::Type = syn::parse_str(&func.result_types.error).unwrap();

            let param_list = if params.is_empty() {
                quote! {}
            } else {
                let params: proc_macro2::TokenStream = params.parse().unwrap();
                quote! { , #params }
            };

            let arg_list = if func.parameters.is_empty() {
                quote! {}
            } else {
                let args = func
                    .parameters
                    .iter()
                    .map(|param| param.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                let args: proc_macro2::TokenStream = args.parse().unwrap();
                quote! { , #args }
            };

            let docstring = &func.documentation;

            let method = quote! {
                #[doc = #docstring]
                pub async fn #fn_name(&self #param_list) -> Result<#return_type, apis::Error<apis::#module_name::#error_type>> {
                    apis::#module_name::#fn_name(&self.config #arg_list).await
                }
            };

            all_methods.push(method);
        }
    }

    quote! {
        #(#all_methods)*
    }
}

/// generates the complete client implementation as a string.
/// this is the final output of the build process that creates
/// a strongly-typed client matching the API's interface.
pub fn generate_client_impl(functions: &[FunctionInfo]) -> String {
    let api_methods = generate_api_methods(functions);

    let impl_block = quote! {
        impl Client {
            #api_methods
        }
    };

    impl_block.to_string()
}

/// prints a formatted build information message during the build process
///
/// useful for debugging the build process and seeing what's happening.
pub fn build_print_info(msg: &str) {
    println!("cargo:warning=\x1b[2K\r\x1b[1m\x1b[34minfo\x1b[0m: {}", msg);
}

/// extracts documentation comments from function attributes to preserve
/// API documentation in the generated client code, to make sure users have
/// the same documentation available as they would with direct API access.
fn parse_enum_doc_comment(attrs: &[syn::Attribute]) -> String {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("doc"))
        .map(|attr| &attr.meta)
        .and_then(|meta| match meta {
            syn::Meta::NameValue(nv) => Some(&nv.value),
            _ => None,
        })
        .and_then(|expr| match expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) => Some(s.value()),
            _ => None,
        })
        .unwrap_or_default()
}
