#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// fetches SDL2 `bindgen` bindings from `target/` at compile time
include!(concat!(env!("OUT_DIR"), "/sdl2_bindings.rs"));
