// build.rs

extern crate bindgen;

use std::process::Command;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=enet/enet.h");
    // println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();

    // Build enet library
    // -> clang -c enet.c -o enet.lib
    Command::new("clang").args(&["enet/enet.c", "-c", "-std=c17", "-o"])
        .arg(&format!("{}/enet.lib", out_dir))
        .status().unwrap();

    // -> clang -shared enet.c -DENET_DLL -O3 -o bin/enet.dll
    // Command::new("clang").args(&["enet/enet.c", "-shared", "-DENET_DLL", "-O3", "-o"])
    //     .arg(&format!("{}/bin/enet.dll", env::var("CARGO_MANIFEST_DIR").unwrap()))
    //     .status().unwrap();

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(&format!("{}/bindings.rs", out_dir))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=static={}", out_dir);
    println!("cargo:rustc-link-lib=enet");
}