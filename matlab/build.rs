use std::env;

fn main() {
    // Path to directory containing "libmex.lib" and "libmx.lib" files
    let matlab_lib_dir = env::var("MATLAB_LIB_DIR")
        .expect("path to MATLAB lib files must be provided in the MATLAB_LIB_DIR env variable");

    println!("cargo:rustc-link-search=native={matlab_lib_dir}");
}
