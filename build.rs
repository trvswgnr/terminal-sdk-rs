use regex::Regex;
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Function info to store parsed API function details
#[derive(Debug)]
struct FunctionInfo {
    module_name: String,
    function_name: String,
    return_type: String,
    error_type: String,
    parameters: Vec<(String, String)>, // (name, type)
}

fn main() {
    let src = "./openapi/src";
    println!("cargo:rerun-if-changed={}", src);

    // Get the output directory from cargo
    let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR");
    let dest_path = Path::new(&out_dir).join("api_methods_gen.rs");
    let mut out_file = File::create(&dest_path).expect("Failed to create output file");

    // Parse API modules and collect function information
    let api_modules = discover_api_modules();
    let functions = parse_api_functions(&api_modules);

    // Generate the impl block with API methods
    writeln!(out_file, "// Generated API methods implementation").unwrap();
    writeln!(out_file, "impl Client {{").unwrap();

    // Add the API methods
    write_api_methods(&mut out_file, &functions);

    // Close the impl block
    writeln!(out_file, "}}").unwrap();

    println!(
        "cargo:warning=\x1b[2K\r   \x1b[1m\x1b[36mInfo:\x1b[0m Generated client for {} API functions",
        functions.len()
    );
}

fn discover_api_modules() -> Vec<String> {
    let apis_dir = Path::new("openapi/src/apis");
    let mut modules = Vec::new();

    for entry in fs::read_dir(apis_dir).expect("Failed to read API directory") {
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

fn parse_api_functions(api_modules: &[String]) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();

    for module_name in api_modules {
        let module_path = format!("openapi/src/apis/{}.rs", module_name);
        let file = File::open(&module_path).expect("Failed to open module file");
        let reader = BufReader::new(file);

        // Use regex to extract function signatures
        let fn_regex = Regex::new(r"pub async fn (\w+)\s*\(\s*configuration:\s*&configuration::Configuration(?:\s*,\s*([^)]+))?\)\s*->\s*Result<([^,]+),\s*Error<([^>]+)>>").unwrap();
        let param_regex = Regex::new(r"(\w+):\s*(?:Option<)?([^,>]+)(?:>)?").unwrap();

        let mut lines = String::new();
        for line in reader.lines() {
            lines.push_str(&line.expect("Failed to read line"));
            lines.push(' ');
        }

        for captures in fn_regex.captures_iter(&lines) {
            let function_name = captures
                .get(1)
                .expect("Failed to get function name")
                .as_str()
                .to_string();
            let return_type = captures
                .get(3)
                .expect("Failed to get return type")
                .as_str()
                .trim()
                .to_string();
            let error_type = captures
                .get(4)
                .expect("Failed to get error type")
                .as_str()
                .trim()
                .to_string();

            let mut parameters = Vec::new();
            if let Some(params_str) = captures.get(2) {
                for param_capture in param_regex.captures_iter(params_str.as_str()) {
                    let param_name = param_capture
                        .get(1)
                        .expect("Failed to get parameter name")
                        .as_str()
                        .to_string();
                    let param_type = param_capture
                        .get(2)
                        .expect("Failed to get parameter type")
                        .as_str()
                        .trim()
                        .to_string();
                    // Determine if the parameter is optional
                    let is_optional = params_str
                        .as_str()
                        .contains(&format!("Option<{}", param_type));
                    let full_type = if is_optional {
                        format!("Option<{}>", param_type)
                    } else {
                        param_type
                    };

                    parameters.push((param_name, full_type));
                }
            }

            functions.push(FunctionInfo {
                module_name: module_name.clone(),
                function_name,
                return_type,
                error_type,
                parameters,
            });
        }
    }

    // Sort functions by module and name for consistent output
    functions.sort_by(|a, b| match a.module_name.cmp(&b.module_name) {
        std::cmp::Ordering::Equal => a.function_name.cmp(&b.function_name),
        other => other,
    });

    functions
}

fn write_api_methods(out_file: &mut File, functions: &[FunctionInfo]) {
    // Group functions by module
    let mut modules: BTreeMap<&str, Vec<&FunctionInfo>> = BTreeMap::new();
    for func in functions {
        modules.entry(&func.module_name).or_default().push(func);
    }

    // Write each function as a method
    for (module, funcs) in &modules {
        writeln!(
            out_file,
            "    // {} API",
            module.replace("_api", "").to_uppercase()
        )
        .unwrap();

        for func in funcs {
            // Write the function signature
            let params = func
                .parameters
                .iter()
                .map(|(name, ty)| format!("{}: {}", name, ty))
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(
                out_file,
                "    pub async fn {}(&self{}) -> Result<{}, apis::Error<apis::{}::{}>> {{",
                func.function_name,
                if params.is_empty() {
                    String::new()
                } else {
                    format!(", {}", params)
                },
                func.return_type,
                func.module_name,
                func.error_type
            )
            .unwrap();

            // Write the function body
            let args = func
                .parameters
                .iter()
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(
                out_file,
                "        apis::{}::{}(&self.config{}).await",
                func.module_name,
                func.function_name,
                if args.is_empty() {
                    String::new()
                } else {
                    format!(", {}", args)
                }
            )
            .unwrap();

            writeln!(out_file, "    }}").unwrap();
            writeln!(out_file).unwrap();
        }
    }
}
