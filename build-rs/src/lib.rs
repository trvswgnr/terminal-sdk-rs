use quote::{ToTokens, quote};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::Path,
};
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
pub struct ApiFunctionInfo {
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
/// - non-rust files
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
) -> Result<Vec<ApiFunctionInfo>, BuildError> {
    let mut seen_functions = HashMap::new();
    let mut functions: Vec<ApiFunctionInfo> = api_modules
        .iter()
        .map(|module_name| {
            let module_path = folder_path.join(format!("{}.rs", module_name));
            let source = fs::read_to_string(&module_path)?;
            parse_module_api_functions(&source, module_name, &mut seen_functions)
        })
        .collect::<Result<Vec<_>, BuildError>>()?
        .into_iter()
        .flatten()
        .collect();

    // sort for deterministic output
    functions.sort_by(|a, b| {
        a.module_name
            .cmp(&b.module_name)
            .then(a.function_name.cmp(&b.function_name))
    });

    Ok(functions)
}

fn parse_module_api_functions(
    code: &str,
    module_name: &str,
    seen_functions: &mut HashMap<String, usize>,
) -> Result<Vec<ApiFunctionInfo>, BuildError> {
    let file = parse_file(code)?;
    let functions = file
        .items
        .into_iter()
        .filter_map(|item| match item {
            syn::Item::Fn(func) => parse_function(func, module_name),
            _ => None,
        })
        // handle possibility of duplicate function names
        .map(|mut func| {
            let fn_name = func.function_name.clone();
            let count = seen_functions.get(&fn_name).unwrap_or(&0);
            if *count > 0 {
                func.function_name = format!("{}_{}", fn_name, count + 1);
            }
            seen_functions.insert(fn_name, count + 1);
            func
        })
        .collect();

    Ok(functions)
}

/// extracts metadata from a function if it meets API requirements:
/// - public visibility for external access
/// - async for non-blocking operation
/// - configuration parameter for API setup
///
/// returns `None` if the function doesn't meet these requirements or
/// if the return type doesn't match the expected `Result<T, Error<E>>` pattern.
fn parse_function(func: ItemFn, module_name: &str) -> Option<ApiFunctionInfo> {
    if !is_valid_api_function(&func) {
        return None;
    }

    let function_name = func.sig.ident.to_string();
    let parameters = extract_parameters(&func.sig.inputs);
    let result_types = extract_result_types(&func.sig.output)?;
    let documentation = parse_enum_doc_comment(&func.attrs);

    Some(ApiFunctionInfo {
        module_name: module_name.to_string(),
        function_name,
        result_types,
        parameters,
        documentation,
    })
}

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
            // skip if not a typed argument
            let FnArg::Typed(pat_type) = arg else {
                return None;
            };

            // skip if not an identifier pattern like `param: Type`
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

    // `Result` must have exactly two type parameters
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

/// generates a module comment from the module name
fn generate_module_comment(module: &str) -> Result<proc_macro2::TokenStream, BuildError> {
    let comment = format!(
        "/// {} API\n///\n",
        module.replace("_api", "").to_uppercase()
    );
    Ok(comment.parse()?)
}

/// generates parameter list for a function
fn generate_param_list(params: &[ParamInfo]) -> Result<proc_macro2::TokenStream, BuildError> {
    if params.is_empty() {
        return Ok(quote! {});
    }

    let param_string = params
        .iter()
        .map(|param| format!("{}: {}", param.name, param.ty))
        .collect::<Vec<_>>()
        .join(", ");

    let params: proc_macro2::TokenStream = param_string.parse()?;
    Ok(quote! { , #params })
}

/// generates argument list for a function call
fn generate_arg_list(params: &[ParamInfo]) -> Result<proc_macro2::TokenStream, BuildError> {
    if params.is_empty() {
        return Ok(quote! {});
    }

    let arg_string = params
        .iter()
        .map(|param| param.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    let args: proc_macro2::TokenStream = arg_string.parse()?;
    Ok(quote! { , #args })
}

/// generates a single API method
fn generate_method(func: &ApiFunctionInfo) -> Result<proc_macro2::TokenStream, BuildError> {
    let fn_name = syn::Ident::new(&func.function_name, proc_macro2::Span::call_site());
    let module_name = syn::Ident::new(&func.module_name, proc_macro2::Span::call_site());
    let return_type: syn::Type = syn::parse_str(&func.result_types.value)?;
    let error_type: syn::Type = syn::parse_str(&func.result_types.error)?;

    let param_list = generate_param_list(&func.parameters)?;
    let arg_list = generate_arg_list(&func.parameters)?;

    let docstring = &func.documentation;

    Ok(quote! {
        #[doc = #docstring]
        pub async fn #fn_name(&self #param_list) -> Result<#return_type, apis::Error<apis::#module_name::#error_type>> {
            apis::#module_name::#fn_name(&self.config #arg_list).await
        }
    })
}

/// generates the implementation block containing all API methods for the client.
/// this creates a properly typed wrapper method for each API function that:
/// - maintains the same parameter types
/// - preserves return types
/// - automatically handles configuration
/// - preserves documentation
fn generate_api_methods(
    functions: &[ApiFunctionInfo],
) -> Result<proc_macro2::TokenStream, BuildError> {
    // group functions by module
    let modules: BTreeMap<&str, Vec<&ApiFunctionInfo>> =
        functions.iter().fold(BTreeMap::new(), |mut acc, func| {
            acc.entry(&func.module_name).or_default().push(func);
            acc
        });

    // generate methods for each module
    let methods: Vec<proc_macro2::TokenStream> = modules.iter().try_fold(
        Vec::new(),
        |mut acc, (module, funcs)| -> Result<Vec<proc_macro2::TokenStream>, BuildError> {
            let module_comment = generate_module_comment(module)?;
            acc.push(module_comment);

            let module_methods = funcs
                .iter()
                .map(|func| generate_method(func))
                .collect::<Result<Vec<_>, BuildError>>()?;

            acc.extend(module_methods);
            Ok(acc)
        },
    )?;

    // combine all methods into a single token stream
    Ok(quote!(#(#methods)*))
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
        .trim()
        .to_string()
}

/// generates the complete client implementation as a string.
/// this is the final output of the build process that creates
/// a strongly-typed client matching the API's interface.
pub fn generate_client_impl(functions: &[ApiFunctionInfo]) -> Result<String, BuildError> {
    let api_methods = generate_api_methods(functions)?;

    let impl_block = quote! {
        impl Client {
            /// Creates a new client with the given configuration
            pub fn new(config: Config) -> Self {
                Client { config }
            }

            #api_methods
        }
    };

    Ok(impl_block.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use syn::parse_quote;
    use tempfile::TempDir;

    #[test]
    fn test_discover_api_module_names() -> Result<(), BuildError> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        fs::write(temp_path.join("valid_api.rs"), "")?;
        fs::write(temp_path.join("another_api.rs"), "")?;
        fs::write(temp_path.join("mod.rs"), "")?;
        fs::write(temp_path.join("configuration.rs"), "")?;
        fs::write(temp_path.join("not_a_rust_file.txt"), "")?;

        let mut module_names = discover_api_module_names(temp_path)?;
        module_names.sort(); // sort for deterministic comparison

        assert_eq!(
            module_names,
            vec!["another_api".to_string(), "valid_api".to_string()]
        );

        temp_dir.close()?;

        Ok(())
    }

    #[test]
    fn test_parse_api_functions() -> Result<(), BuildError> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        fn do_test(functions: &[ApiFunctionInfo]) {
            assert_eq!(functions.len(), 1);
            let func = &functions[0];
            assert_eq!(func.module_name, "test_api");
            assert_eq!(func.function_name, "test_function");
            assert_eq!(func.parameters.len(), 2);
            assert_eq!(func.parameters[0].name, "param1");
            assert_eq!(func.parameters[0].ty, "String");
            assert_eq!(func.parameters[1].name, "param2");
            assert_eq!(func.parameters[1].ty, "i32");
            assert_eq!(func.result_types.value, "Vec < String >");
            assert_eq!(func.result_types.error, "CustomError");
            assert_eq!(func.documentation, "Test function documentation");
        }

        // test with a valid function
        let api_content = r#"
            /// Test function documentation
            pub async fn test_function(
                configuration: &Configuration,
                param1: String,
                param2: i32,
            ) -> Result<Vec<String>, Error<CustomError>> {
                todo!()
            }
        "#;
        fs::write(temp_path.join("test_api.rs"), api_content)?;

        let functions = parse_api_functions(temp_path, &["test_api".to_string()])?;

        do_test(&functions);

        // test with a valid function with weird whitespace
        let api_content = r#"
            /// Test function documentation
            pub async 
            fn test_function     (
                configuration: &Configuration,
                param1:
        String,
                param2   :   i32,
            ) -> Result<
             Vec<String>, Error<
             CustomError>       > {
                todo!   ()
            }
        "#;
        fs::write(temp_path.join("test_api.rs"), api_content)?;

        let functions = parse_api_functions(temp_path, &["test_api".to_string()])?;

        do_test(&functions);

        // clean up
        temp_dir.close()?;

        Ok(())
    }

    #[test]
    fn test_parse_api_functions_with_duplicate_names() -> Result<(), BuildError> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        let api_content = r#"
            /// Test function documentation
            pub async fn test_function(configuration: &Configuration) -> Result<(), Error<()>> {
                todo!()
            }
        "#;
        fs::write(temp_path.join("test_api.rs"), api_content)?;

        let api_content2 = r#"
            /// Test function documentation
            pub async fn test_function(configuration: &Configuration) -> Result<(), Error<()>> {
                todo!()
            }
        "#;
        fs::write(temp_path.join("test_api2.rs"), api_content2)?;

        let functions = parse_api_functions(
            temp_path,
            &["test_api".to_string(), "test_api2".to_string()],
        )?;
        assert_eq!(functions.len(), 2);
        assert_eq!(functions[0].function_name, "test_function");
        assert_eq!(functions[1].function_name, "test_function_2");

        // clean up
        temp_dir.close()?;

        Ok(())
    }

    #[test]
    fn test_parse_api_functions_with_errors() -> Result<(), BuildError> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // with syntax error
        fs::write(
            temp_path.join("test_api.rs"),
            "pub async fn test_function() -> Result<(), Error<()>> { todo!() }",
        )?;

        let functions = parse_api_functions(temp_path, &["test_api".to_string()])?;
        assert!(functions.is_empty());

        // with valid function but an invalid return type
        fs::write(
            temp_path.join("test_api.rs"),
            "pub async fn test_function() -> Result<(), Error<()>> { todo!() }",
        )?;

        let functions = parse_api_functions(temp_path, &["test_api".to_string()])?;
        assert!(functions.is_empty());

        // clean up
        temp_dir.close()?;

        Ok(())
    }

    #[test]
    fn test_is_valid_api_function() {
        // valid API function
        let valid_fn: ItemFn = parse_quote! {
            pub async fn valid_api(configuration: &Configuration) -> Result<(), Error<()>> {
                todo!()
            }
        };
        assert!(is_valid_api_function(&valid_fn));

        // not public
        let private_fn: ItemFn = parse_quote! {
            async fn private_api(configuration: &Configuration) -> Result<(), Error<()>> {
                todo!()
            }
        };
        assert!(!is_valid_api_function(&private_fn));

        // not async
        let sync_fn: ItemFn = parse_quote! {
            pub fn sync_api(configuration: &Configuration) -> Result<(), Error<()>> {
                todo!()
            }
        };
        assert!(!is_valid_api_function(&sync_fn));

        // no configuration parameter
        let no_config_fn: ItemFn = parse_quote! {
            pub async fn no_config_api() -> Result<(), Error<()>> {
                todo!()
            }
        };
        assert!(!is_valid_api_function(&no_config_fn));
    }

    #[test]
    fn test_extract_parameters() {
        let fn_sig: syn::Signature = parse_quote! {
            async fn test(
                configuration: &Configuration,
                param1: String,
                param2: Vec<i32>,
            ) -> Result<(), Error<()>>
        };

        let params = extract_parameters(&fn_sig.inputs);
        assert_eq!(params.len(), 2); // configuration should be filtered out
        assert_eq!(params[0].name, "param1");
        assert_eq!(params[0].ty, "String");
        assert_eq!(params[1].name, "param2");
        assert_eq!(params[1].ty, "Vec < i32 >");
    }

    #[test]
    fn test_extract_result_types() {
        // simple types
        let return_type: ReturnType = parse_quote! {
            -> Result<String, Error<CustomError>>
        };
        let result = extract_result_types(&return_type).unwrap();
        assert_eq!(result.value, "String");
        assert_eq!(result.error, "CustomError");

        // complex generic types
        let complex_return: ReturnType = parse_quote! {
            -> Result<Vec<HashMap<String, i32>>, Error<ComplexError>>
        };
        let result = extract_result_types(&complex_return).unwrap();
        assert_eq!(result.value, "Vec < HashMap < String , i32 > >");
        assert_eq!(result.error, "ComplexError");

        // invalid return type (not a Result)
        let invalid_return: ReturnType = parse_quote! {
            -> String
        };
        assert!(extract_result_types(&invalid_return).is_none());
    }

    #[test]
    fn test_generate_api_methods() {
        let functions = vec![ApiFunctionInfo {
            module_name: "test_api".to_string(),
            function_name: "test_function".to_string(),
            result_types: ResultTypesInfo {
                value: "String".to_string(),
                error: "TestError".to_string(),
            },
            parameters: vec![ParamInfo {
                name: "param1".to_string(),
                ty: "i32".to_string(),
            }],
            documentation: "/// Test function documentation".to_string(),
        }];

        let generated = generate_api_methods(&functions).unwrap().to_string();
        assert!(generated.contains("test_function"));
        assert!(generated.contains("param1 : i32"));
        assert!(
            generated
                .contains("Result < String , apis :: Error < apis :: test_api :: TestError >>")
        );
        assert!(generated.contains("Test function documentation"));
    }

    #[test]
    fn test_parse_enum_doc_comment() {
        let attrs: Vec<syn::Attribute> = parse_quote! {
            #[doc = "Test documentation"]
        };
        assert_eq!(parse_enum_doc_comment(&attrs), "Test documentation");

        // empty attributes
        assert_eq!(parse_enum_doc_comment(&[]), "");

        // non-doc attribute
        let non_doc_attrs: Vec<syn::Attribute> = parse_quote! {
            #[derive(Debug)]
        };
        assert_eq!(parse_enum_doc_comment(&non_doc_attrs), "");
    }

    #[test]
    fn test_generate_client_impl() {
        let functions = vec![ApiFunctionInfo {
            module_name: "test_api".to_string(),
            function_name: "test_function".to_string(),
            result_types: ResultTypesInfo {
                value: "String".to_string(),
                error: "TestError".to_string(),
            },
            parameters: vec![],
            documentation: "Test function".to_string(),
        }];

        let impl_str = generate_client_impl(&functions).unwrap();
        assert!(impl_str.contains("impl Client"));
        assert!(impl_str.contains("test_function"));
        assert!(
            impl_str.contains("Result < String , apis :: Error < apis :: test_api :: TestError >>")
        );
        assert!(impl_str.contains("Test function"));
    }
}
