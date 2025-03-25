use quote::{ToTokens, quote};
use std::{collections::BTreeMap, fs, path::Path, str::FromStr};
use syn::{FnArg, ItemFn, Pat, ReturnType, Type, parse_file, punctuated::Punctuated};

pub type BuildError = Box<dyn std::error::Error>;

/// parameter with name and type as strings
#[derive(Debug)]
struct ParamInfo {
    name: String,
    ty: String,
}

/// result types as strings
#[derive(Debug)]
struct ResultTypesInfo {
    value: String,
    error: String,
}

/// function info to store parsed API function details
#[derive(Debug)]
pub struct FunctionInfo {
    module_name: String,
    function_name: String,
    return_type: String,
    error_type: String,
    parameters: Vec<ParamInfo>,
    documentation: String,
}

pub fn get_api_module_names(folder_path: &Path) -> Result<Vec<String>, BuildError> {
    let names = fs::read_dir(folder_path)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let is_rs_file = path.extension()?.to_str()? == "rs";

            if !is_rs_file {
                return None;
            }

            let file_name = path.file_stem()?.to_string_lossy().into_owned();

            // Skip mod.rs and configuration.rs
            (!["mod", "configuration"].contains(&file_name.as_str())).then_some(file_name)
        })
        .collect();

    Ok(names)
}

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

fn parse_function(func: ItemFn, module_name: &str) -> Option<FunctionInfo> {
    // Only process public async functions with a configuration parameter
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
        return_type: result_types.value,
        error_type: result_types.error,
        parameters,
        documentation,
    })
}

fn is_valid_api_function(func: &ItemFn) -> bool {
    let is_public = matches!(func.vis, syn::Visibility::Public(_));
    let is_async = func.sig.asyncness.is_some();

    // an api function should have a "configuration" param
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

fn extract_result_types(return_type: &ReturnType) -> Option<ResultTypesInfo> {
    let ReturnType::Type(_, ty) = return_type else {
        return None;
    };
    let Type::Path(type_path) = &**ty else {
        return None;
    };

    let last_segment = type_path.path.segments.last()?;
    if last_segment.ident != "Result" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(generic_args) = &last_segment.arguments else {
        return None;
    };
    let args: Vec<_> = generic_args.args.iter().collect();
    if args.len() != 2 {
        return None;
    }

    let syn::GenericArgument::Type(return_type) = args[0] else {
        return None;
    };
    let return_type = return_type.to_token_stream().to_string();

    let syn::GenericArgument::Type(error_type_path) = args[1] else {
        return None;
    };
    let Type::Path(error_path) = error_type_path else {
        return None;
    };

    let error_segment = error_path.path.segments.last()?;
    if error_segment.ident != "Error" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(error_args) = &error_segment.arguments else {
        return None;
    };

    let error_type = error_args.args.first()?.to_token_stream().to_string();

    Some(ResultTypesInfo {
        value: return_type,
        error: error_type,
    })
}

pub fn generate_api_methods(functions: &[FunctionInfo]) -> proc_macro2::TokenStream {
    let mut all_methods = Vec::new();

    // group functions by module
    let mut modules: BTreeMap<&str, Vec<&FunctionInfo>> = BTreeMap::new();
    for func in functions {
        modules.entry(&func.module_name).or_default().push(func);
    }

    // generate each function as a method
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
            let return_type: syn::Type = syn::parse_str(&func.return_type).unwrap();
            let error_type: syn::Type = syn::parse_str(&func.error_type).unwrap();

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

pub fn generate_client_impl(functions: &[FunctionInfo]) -> String {
    let api_methods = generate_api_methods(functions);

    let impl_block = quote! {
        // generated API methods implementation
        impl Client {
            #api_methods
        }
    };

    impl_block.to_string()
}

pub fn build_print_info(msg: &str) {
    println!("cargo:warning=\x1b[2K\r\x1b[1m\x1b[34minfo\x1b[0m: {}", msg);
}

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
