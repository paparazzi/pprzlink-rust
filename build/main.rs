#![recursion_limit="256"]
#[macro_use]
extern crate quote;

use std::env;
use std::fs::File;
use std::path::Path;

mod parser;

fn main() {
    let src_dir = env::current_dir().unwrap();
    let in_path = Path::new(&src_dir).join("common.xml");
    let mut inf = File::open(&in_path).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path_rust = Path::new(&out_dir);
    parser::generate(&mut inf, &dest_path_rust);

    // Dont run build.rs unless it is changed
    //println!("cargo:rerun-if-changed=build.rs");
}
