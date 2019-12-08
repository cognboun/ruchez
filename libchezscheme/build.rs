extern crate bindgen;

use std::env;
use std::path::PathBuf;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn get_lib_name() -> Option<String> {
    let path = Path::new("chezscheme_lib_name");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(_) => return None,
        Ok(file) => file
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => None,
        Ok(_) => Some(s)
    }
}


const LIB_NAME: &str = "kernel";
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

    let lib_name = if let Some(lib_name) = get_lib_name() {
	lib_name
    } else {
	String::from(LIB_NAME)
    };

    println!("cargo:rustc-link-search={}", lib_path.display());
    //println!("cargo:rustc-link-lib=static={}", LIB_NAME);
    println!("cargo:rustc-link-lib={}", lib_name);
    println!("cargo:rustc-link-lib=z"); // default =dylib
    println!("cargo:rustc-link-lib=iconv"); // default =dylib
    println!("cargo:rustc-link-lib=ncurses"); // default =dylib
}
