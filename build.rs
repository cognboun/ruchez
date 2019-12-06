extern crate bindgen;

use std::env;
use std::path::PathBuf;

const LIB_NAME: &str = "chezscheme";
const LIB_PATH: &str = "chezscheme";

fn main() {

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("unable to generate hello bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");

    let lib_path = PathBuf::from(env::current_dir().unwrap().join(LIB_PATH));

    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=static={}", LIB_NAME);
}
