#[macro_use]
extern crate serde_derive;
extern crate serde;

/// Pprzlink message set
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod pprzlink {
    include!("../common.rs");
}

use crate::pprzlink::alert;

fn main() {
    println!("Hello rust");
}