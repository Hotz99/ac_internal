/*
* This hack is relatively simple. It is loaded into the AssaultCube process through
* the LD_PRELOAD technique (e.g.) LD_PRELOAD=./hack.so ./assaultcube.sh in the main AC directory.
* There is a constructor, which runs at load time. It is used to initialize the hack by
*  - verifying this library is actually loaded into the game and not for example /bin/sh when
        launching AC through ./assaultcube.sh
*  - finding offsets of code to patch
*  - generating shellcode on the fly through nasm for hooks
*  - prepares hooks
*  - initialized the global AC_HACK variable
*  - dynamically loads libSDL and obtains a pointer to SDL_GL_SwapWindow()
*
*  By using the LD_PRELOAD technique, this hack hooks SDL_GL_SwapWindow().
*  This function will then use the initialized, static variable AC_HACK to perform the logic
*  it needs to do such as getting player positions, draw ESP boxes etc.
*  The reason we use statics here is that we don't want to reload the entire hack
*  for each frame
*/

pub mod game;
pub mod hacks;
pub mod instance;
pub mod offsets;
pub mod player;
pub mod process;
pub mod utils;

extern crate ctor;
use ctor::ctor;

// static reference to hack instance
// instantiated on load()
// used every frame on SDL_GL_SwapBuffers()
static mut INSTANCE: Option<instance::Instance> = None;

const GAME_MODULE: &str = "linux_64_client";

// reference to the dynamiclly loaded libSDL2
// used to resolve the address of the original SDL_GL_SwapBuffers()
// when unhooking
static mut SDL2_DYLIB: Option<libloading::Library> = None;

// called when shared object is loaded
#[ctor]
fn load() {
    let process = process::Process::get_current().expect("failed to read /proc/self/maps");

    if let Err(_e) = process.get_module(GAME_MODULE) {
        return;
    }

    let mut found_lib_sdl2 = false;
    let modules = process
        .get_all_modules()
        .expect("failed to parse the loaded modules");

    for module_name in modules.keys() {
        if module_name.contains("libSDL2") {
            unsafe {
                SDL2_DYLIB =
                    Some(libloading::Library::new(module_name).expect("failed to load libSDL2"))
            };

            found_lib_sdl2 = true;
        }
    }

    if !found_lib_sdl2 {
        panic!("failed to find libSDL2 in current process");
    }

    println!("loaded instance into assaultcube");

    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(3));

        unsafe {
            INSTANCE = Some(instance::Instance::init());
        }
    });
}

fn forward_to_original_sdl_swap_window(
    window: *mut utils::bindings::sdl2_bindings::SDL_Window,
) -> i64 {
    unsafe {
        // hook into the function in the external library, not the one in the current process
        let original_sdl_swap_window: libloading::Symbol<
            unsafe extern "C" fn(window: *mut utils::bindings::sdl2_bindings::SDL_Window) -> i64,
        > = SDL2_DYLIB
            .as_ref()
            .unwrap()
            .get(b"SDL_GL_SwapWindow\0")
            .expect("failed to find SDL_GL_SwapWindow() in libSDL2");

        // ensure the original function args are correct
        // spent too long debugging this when all I had to do was read SDL2 docs
        original_sdl_swap_window(window)
    }
}

#[no_mangle]
pub extern "C" fn SDL_GL_SwapWindow(
    window: *mut utils::bindings::sdl2_bindings::SDL_Window,
) -> i64 {
    let instance = unsafe { &mut INSTANCE.as_ref() };

    if instance.is_none() {
        return forward_to_original_sdl_swap_window(window);
    }

    (*instance).unwrap().esp.draw();
    //(*instance).unwrap().aimbot.logic();

    forward_to_original_sdl_swap_window(window)
}
