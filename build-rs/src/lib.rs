use quote::{ToTokens, quote};
use std::{collections::BTreeMap, fs, path::Path, str::FromStr};
use syn::{FnArg, ItemFn, Pat, ReturnType, Type, parse_file};

/// function info to store parsed API function details
#[derive(Debug)]
pub struct FunctionInfo {
    module_name: String,
    function_name: String,
    return_type: String,
    error_type: String,
    parameters: Vec<(String, String)>, // (name, type)
    documentation: String,
}

pub fn get_api_module_names(folder_path: &Path) -> Vec<String> {
    let mut modules = Vec::new();

    for entry in fs::read_dir(folder_path).expect("Failed to read API directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == "rs" {
            let file_name = path
                .file_stem()
                .expect("Failed to get file name")
                .to_str()
                .expect("Failed to convert file name to str");

            if file_name != "mod" && file_name != "configuration" {
                modules.push(file_name.to_string());
            }
        }
    }

    modules
}

pub fn parse_api_functions(folder_path: &Path, api_modules: &[String]) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();

    for module_name in api_modules {
        let module_path = format!("{}/{}.rs", folder_path.display(), module_name);
        let source = fs::read_to_string(&module_path).expect("Failed to read module file");
        let syntax = parse_file(&source).expect("Failed to parse module file");

        for item in syntax.items {
            if let syn::Item::Fn(func) = item {
                if let Some(function_info) = parse_function(func, module_name) {
                    functions.push(function_info);
                }
            }
        }
    }

    // sort functions by module and name for consistent output
    functions.sort_by(|a, b| match a.module_name.cmp(&b.module_name) {
        std::cmp::Ordering::Equal => a.function_name.cmp(&b.function_name),
        other => other,
    });

    functions
}

fn parse_function(func: ItemFn, module_name: &str) -> Option<FunctionInfo> {
    // check if function is public and async
    if !matches!(func.vis, syn::Visibility::Public(_)) || func.sig.asyncness.is_none() {
        return None;
    }

    let function_name = func.sig.ident.to_string();

    // parse parameters, excluding configuration
    let mut parameters = Vec::new();
    let mut found_config = false;

    for arg in func.sig.inputs.iter() {
        let FnArg::Typed(pat_type) = arg else {
            continue;
        };
        let Pat::Ident(pat_ident) = &*pat_type.pat else {
            continue;
        };

        let param_name = pat_ident.ident.to_string();
        if param_name == "configuration" {
            found_config = true;
            continue;
        }

        let param_type = pat_type.ty.to_token_stream().to_string();
        parameters.push((param_name, param_type));
    }

    if !found_config {
        return None;
    }

    let ReturnType::Type(_, ty) = &func.sig.output else {
        return None;
    };

    let Type::Path(type_path) = &**ty else {
        return None;
    };

    // make sure we have a Result type with exactly one segment
    let last_segment = type_path.path.segments.last()?;
    if last_segment.ident != "Result" {
        return None;
    }

    // get the generic arguments inside Result<T, Error<E>>
    let syn::PathArguments::AngleBracketed(generic_args) = &last_segment.arguments else {
        return None;
    };
    let args = generic_args.args.iter().collect::<Vec<_>>();
    if args.len() != 2 {
        return None;
    }

    // extract return type T from Result<T, Error<E>>
    let syn::GenericArgument::Type(return_type) = args[0] else {
        return None;
    };
    let return_type = return_type.to_token_stream().to_string();

    // extract error type E from Error<E>
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

    let documentation = parse_enum_doc_comment(&func.attrs);

    Some(FunctionInfo {
        module_name: module_name.to_string(),
        function_name,
        return_type,
        error_type,
        parameters,
        documentation,
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
                .map(|(name, ty)| format!("{}: {}", name, ty))
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
                    .map(|(name, _)| name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                let args: proc_macro2::TokenStream = args.parse().unwrap();
                quote! { , #args }
            };

            let documentation = func.documentation.clone();

            let method = quote! {
                #[doc = #documentation]
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
    match attrs.iter().find(|attr| attr.path().is_ident("doc")) {
        Some(attr) => match &attr.meta {
            syn::Meta::NameValue(name_value) => match &name_value.value {
                syn::Expr::Lit(lit) => match &lit.lit {
                    syn::Lit::Str(lit_str) => lit_str.value(),
                    _ => String::new(),
                },
                _ => String::new(),
            },
            _ => String::new(),
        },
        None => String::new(),
    }
}
