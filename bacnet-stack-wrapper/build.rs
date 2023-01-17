extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Stdio;

fn main() {
    
    // println!("cargo:rustc-link-lib=bz2");

    println!("sdklfhjdfkjghsdfkgjlhdfgkjdsfhgsdkfjhsgd");

    let result = std::process::Command::new("make")
        .current_dir("./bacnet-stack")
        .spawn()
        .expect("makefile failed");

    // let bindings = bindgen::Builder::default()
    //     .header("wrapper.h")
    //     // .clang_arg("-F/usr/lib/gcc/x86_64-linux-gnu/9/include")
    //     // .clang_arg("-Fbacnet-stack/src")
    //     .clang_arg("-Ibacnet-stack/src")
    //     .blocklist_item("FP_INFINITE")
    //     .blocklist_item("FP_NAN")
    //     .blocklist_item("FP_ZERO")
    //     .blocklist_item("FP_NORMAL")
    //     .blocklist_item("FP_SUBNORMAL")
    //     .generate()
    //     .expect("Unable to generate bindings");

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}