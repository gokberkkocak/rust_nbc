extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(feature = "generate-bindings")]
    generate_bindings();
    build();
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(){
    let bindings = bindgen::Builder::default()
        .header("vendor/wrapper.h")
        .rustfmt_bindings(true)
        .clang_arg(r"-std=c99")
        .whitelist_type("solver*")
        .whitelist_function("solver.*")
        .whitelist_type("clause*")
        .whitelist_type("lit*")
        .whitelist_type("veci*")
        .whitelist_function("veci.*")
        .whitelist_type("vecp*")
        .whitelist_function("vecp.*")
        // this 2 since it just makes it look nicer.
        // .use_core()
        // .ctypes_prefix("libc")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("nbc_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build() {
    cc::Build::new()
        .include("vendor")
        .file("vendor/solver.c")
        .file("vendor/vec.c")
        .flag_if_supported("DBJ")
        .flag_if_supported("DNDEBUG")
        .flag_if_supported("DDLEVEL")
        .flag_if_supported("fomit-frame-pointer")
        .flag_if_supported(r"std=c99")
        .flag_if_supported("-w")
        .opt_level(3)
        .compile("nbcallsat");
}