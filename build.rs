use std::{env, fs::File, io::Write, path::Path};

use build_rs::{build_print_info, generate_client_impl, get_api_module_names, parse_api_functions};

fn main() {
    let apis_folder = Path::new("./openapi/src/apis");
    println!("cargo:rerun-if-changed={}", apis_folder.display());

    let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR");
    let dest_path = Path::new(&out_dir).join("api_methods_gen.rs");
    let mut out_file = File::create(&dest_path).expect("Failed to create output file");
    let api_modules = get_api_module_names(apis_folder);
    let functions = parse_api_functions(apis_folder, &api_modules);
    let impl_block = generate_client_impl(&functions);
    out_file
        .write_all(impl_block.as_bytes())
        .expect("Failed to write to output file");

    build_print_info(&format!(
        "Generated client for {} API functions",
        functions.len()
    ));
}
