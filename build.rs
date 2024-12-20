use std::env;
use std::path::PathBuf;

const GL_HEADER: &str = "src/utils/bindings/gl_header.h";
const GL_BINDINGS_FILE: &str = "gl_bindings.rs";
const SDL2_HEADER: &str = "src/utils/bindings/sdl2_header.h";
const SDL2_BINDINGS_FILE: &str = "sdl2_bindings.rs";

fn main() {
    // tell rustc to link libGL
    // needed for drawing
    println!("cargo:rustc-link-lib=GL");

    // tell cargo to re-build if the gl_bindings.h changes
    println!("cargo:rerun-if-changed={}", GL_HEADER);

    // generate rust bindings for libGL
    let bindings = bindgen::Builder::default()
        .header(GL_HEADER)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate GL bindings");

    // write bindings to $OUT_DIR/gl_bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(GL_BINDINGS_FILE))
        .expect("failed to write GL bindings");

    // tell rustc to link libSDL2
    // needed for hooking
    println!("cargo:rustc-link-lib=SDL2");

    // tell cargo to re-build if the sdl_bindings.h changes
    println!("cargo:rerun-if-changed={}", SDL2_HEADER);

    // generate rust bindings for libSDL2
    let bindings = bindgen::Builder::default()
        .header(SDL2_HEADER)
        .allowlist_type("SDL_Window")
        .allowlist_type("SDL_Scancode")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate SDL2 bindings");

    // write bindings to $OUT_DIR/sdl2_bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(SDL2_BINDINGS_FILE))
        .expect("failed to write SDL2 bindings");
}
