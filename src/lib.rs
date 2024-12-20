pub mod game;
pub mod hacks;
pub mod instance;
pub mod offsets;
pub mod player;
pub mod process;
pub mod utils;

extern crate ctor;
use ctor::ctor;
use utils::bindings::sdl2_bindings;

// static reference to hack instance
// instantiated on load()
// used every frame on SDL_GL_SwapBuffers()
// shared between main and hook threads
pub static mut INSTANCE: Option<instance::Instance> = None;
const GAME_MODULE: &str = "linux_64_client";

// reference to the dynamically loaded libSDL2
// used to resolve the address of the original SDL_GL_SwapBuffers() when unhooking
static mut SDL2_DYLIB: Option<libloading::Library> = None;

const INPUT_POLLING_TIMEOUT_MS: u64 = 150;

// called when this lib is loaded
#[ctor]
fn load() {
    let process = process::Process::get_current().expect("failed to read /proc/{self}/maps");

    if let Err(_e) = process.get_module(GAME_MODULE) {
        return;
    }

    let modules = process
        .get_all_modules()
        .expect("failed to parse the loaded modules");

    let module_name = modules
        .keys()
        .find(|name| name.contains("libSDL2"))
        .expect("failed to find libSDL2");

    unsafe {
        SDL2_DYLIB = Some(libloading::Library::new(module_name).expect("failed to load libSDL2"))
    };

    println!("[AC_INTERNAL] loaded instance into assaultcube");

    std::thread::spawn(|| unsafe {
        INSTANCE = Some(instance::Instance::default());

        // nasty work, but wayland makes it hard to get keyboard input
        // so we resort to polling SDL2
        loop {
            if is_key_pressed(sdl2_bindings::SDL_Scancode_SDL_SCANCODE_F1) {
                if INSTANCE.as_mut().expect("instance is None").esp.toggle() {
                    println!("[AC_INTERNAL] enabled ESP");
                } else {
                    println!("[AC_INTERNAL] disabled ESP");
                }
            } else if is_key_pressed(sdl2_bindings::SDL_Scancode_SDL_SCANCODE_F2) {
                if INSTANCE
                    .as_mut()
                    .expect("instance is None")
                    .god_mode
                    .toggle()
                {
                    println!("[AC_INTERNAL] enabled god mode");
                } else {
                    println!("[AC_INTERNAL] disabled god mode");
                }
            } else if is_key_pressed(sdl2_bindings::SDL_Scancode_SDL_SCANCODE_F3) {
                if INSTANCE
                    .as_mut()
                    .expect("instance is None")
                    .infinite_ammo
                    .toggle()
                {
                    println!("[AC_INTERNAL] enabled infinite ammo");
                } else {
                    println!("[AC_INTERNAL] disabled infinite ammo");
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(INPUT_POLLING_TIMEOUT_MS));
        }
    });
}

fn forward_to_original_sdl_swap_window(window: *mut sdl2_bindings::SDL_Window) -> i64 {
    unsafe {
        if SDL2_DYLIB.as_ref().is_none() {
            return 0;
        }
        // hook into the function in the external library, not the one in the current process
        let original_sdl_swap_window: libloading::Symbol<
            unsafe extern "C" fn(window: *mut sdl2_bindings::SDL_Window) -> i64,
        > = SDL2_DYLIB
            .as_ref()
            .unwrap()
            .get(b"SDL_GL_SwapWindow\0")
            .expect("failed to find SDL_GL_SwapWindow() in libSDL2");

        // read the docs to ensure args are correct
        // https://wiki.libsdl.org/SDL2/SDL_GL_SwapWindow
        original_sdl_swap_window(window)
    }
}

// instruct compiler to not mangle the function name
// so that it can be called from the C side (game)
#[no_mangle]
pub extern "C" fn SDL_GL_SwapWindow(window: *mut sdl2_bindings::SDL_Window) -> i64 {
    let mut instance = unsafe { INSTANCE.as_mut() };

    if instance.is_none() || !instance.as_ref().unwrap().esp.is_enabled {
        return forward_to_original_sdl_swap_window(window);
    }

    // this current logic being executed is "inside" the game loop
    // simply direct flow to ESP drawing logic
    instance.as_mut().unwrap().esp.draw();

    forward_to_original_sdl_swap_window(window)
}

// https://wiki.libsdl.org/SDL2/SDL_GetKeyboardState
fn sdl_get_keyboard_state(numkeys: *mut i32) -> *const u8 {
    unsafe {
        let original_sdl_get_keyboard_state: libloading::Symbol<
            unsafe extern "C" fn(*mut i32) -> *const u8,
        > = SDL2_DYLIB
            .as_ref()
            .unwrap()
            .get(b"SDL_GetKeyboardState\0")
            .expect("failed to find SDL_GetKeyboardState() in libSDL2");

        original_sdl_get_keyboard_state(numkeys)
    }
}

pub fn is_key_pressed(scancode: sdl2_bindings::SDL_Scancode) -> bool {
    unsafe {
        let mut keyboard_state_len: i32 = 0;
        let keyboard_state_ptr = sdl_get_keyboard_state(&mut keyboard_state_len);

        // form slice of keyboard state buffer
        let keyboard_state_buffer =
            std::slice::from_raw_parts(keyboard_state_ptr, keyboard_state_len as usize);

        if keyboard_state_buffer.is_empty() {
            return false;
        }

        keyboard_state_buffer[scancode as usize] != 0
    }
}
