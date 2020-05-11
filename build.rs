extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::{
    env,
    path::PathBuf,
};


fn main() {
    let dst = Config::new("libsignal-protocol-c")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
        // Tell rustc to use nng static library
    println!("cargo:rustc-link-lib=static=signal-protocol-c");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Ilibsignal-protocol-c/src/")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

