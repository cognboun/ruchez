use std::env;
use ruchez::{flonum, repl};

fn main() {
    for argument in env::args() {
        println!("*** args = {}", argument);
    }    

    let petite_boot = "./libchezscheme/chezscheme/petite.boot";
    let scheme_boot = "./libchezscheme/chezscheme/scheme.boot";
    println!("hello Chezscheme!!");
    repl(petite_boot, scheme_boot);
    println!("bye-bye!!")
}
