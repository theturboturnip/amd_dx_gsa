extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("assets/AmdDxGsaCompile.h")
        .dynamic_library_name("atidxx64")
        .dynamic_link_require_all(true)
        // Only generate bindings for the compile/free functions (and thus all types they use)
        .allowlist_function("AmdDxGsaCompileShader")
        .allowlist_function("AmdDxGsaFreeCompiledShader")
        .allowlist_type("_AmdDxGsaCompileOptionEnum")
        .newtype_enum("_AmdDxGsaCompileOptionEnum")
        .newtype_enum("AmdDxGsaCompileOptionEnum")
        // Don't reload on dependent headers - the only dependent header is windows.h, which is used for very basic type aliases
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("amd_dx_gsa_compile_bindings.rs"))
        .expect("Couldn't write bindings!");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("assets/devices.h")
        // Use signed ints for #defined ints
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("device_bindings.rs"))
        .expect("Couldn't write bindings!");
}
