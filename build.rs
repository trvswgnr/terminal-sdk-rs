use std::{env, fs::File, io::Write, path::Path};

use build_rs::{
    build_print_info, discover_api_module_names, generate_client_impl, parse_api_functions,
    BuildError,
};

fn main() -> Result<(), BuildError> {
    let apis_folder = Path::new("./openapi/src/apis");
    println!("cargo:rerun-if-changed={}", apis_folder.display());

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("api_methods_gen.rs");
    let api_modules = discover_api_module_names(apis_folder)?;
    let functions = parse_api_functions(apis_folder, &api_modules)?;
    let impl_block = generate_client_impl(&functions)?;

    File::create(&dest_path)?.write_all(impl_block.as_bytes())?;

    build_print_info(&format!(
        "Generated client for {} API functions",
        functions.len()
    ));

    Ok(())
}
