#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// fetches GL bindgen bindings from `target/` at compile time
include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
