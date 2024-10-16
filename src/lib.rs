/**
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
use std::thread;
use std::time::Duration;

extern crate ctor;
extern crate libloading;
use ctor::ctor;

pub mod aimbot;
pub mod esp;
pub mod offsets;
pub mod player;
pub mod process;
pub mod utils;

pub use aimbot::*;
pub use esp::*;
pub use player::*;
pub use process::*;
pub use utils::*;

/// static reference to hack instance
/// instantiated on load()
/// used every frame on SDL_GL_SwapBuffers()
static mut AC_HACK: Option<AcHack> = None;

/// reference to the dynamiclly loaded libSDL2
/// used to resolve the address of the original SDL_GL_SwapBuffers()
/// when unhooking
static mut SDL2_DYLIB: Option<libloading::Library> = None;

#[allow(dead_code)]
struct AcHack {
    pub game_base: usize,
    pub player: Player,

    pub god_mode: GodMode,

    /// hooks the shooting function and enables infinite ammo
    pub infinite_ammo: InfiniteAmmo,

    pub aimbot: AimBot,

    pub esp: ESP,
}

impl AcHack {
    fn default() -> Self {
        // get a handle to the current process
        let player = Player::get_local_player();
        AcHack {
            game_base: process::target::resolve_base_address()
                .expect("failed to resolve game base"),
            aimbot: AimBot::default(),
            esp: ESP::default(),
            god_mode: GodMode::default(),
            infinite_ammo: InfiniteAmmo::default(),
            player,
        }
    }

    fn init() -> Self {
        let mut hack = Self::default();

        // all the following are default settings for this hack
        //hack.aimbot.enable();
        //hack.aimbot.norecoil_spread.toggle();
        //hack.aimbot.enable_autoshoot();
        hack.infinite_ammo.toggle();
        //hack.god_mode.toggle();

        hack
    }
}

/// This function is executed when the hack is loaded into the game
/// it is used to initialize the hack, launch a new thread that listens for keyboard bindings etc
#[ctor]
fn load() {
    // check if current process has a linux_64_client module
    let process = Process::current().expect("failed to read /proc/self/maps");

    if let Err(_e) = process.module("linux_64_client") {
        return;
    }

    let mut found_lib_sdl2 = false;
    let modules = process
        .modules()
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

    println!("Successfully loaded the hack into the game...");
    println!("Waiting 5 seconds for the game to initialize it self before touching anything.");

    thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));

        unsafe {
            AC_HACK = Some(AcHack::init());
        }
    });
}

fn forward_to_original_sdl_swap_window() -> i64 {
    unsafe {
        // hook into the function in the external library, not the one in the current process
        let original_sdl_swap_window: libloading::Symbol<unsafe extern "C" fn() -> i64> =
            SDL2_DYLIB
                .as_ref()
                .unwrap()
                .get(b"SDL_GL_SwapWindow\0")
                .expect("failed to find SDL_GL_SwapWindow() in libSDL2");

        original_sdl_swap_window()
    }
}

//#[no_mangle]
//pub extern "C" fn SDL_GL_SwapWindow() -> i64 {
//    let mut hack = unsafe { &mut AC_HACK };
//
//    if hack.is_none() {
//        return forward_to_original_sdl_swap_window();
//    }
//
//    //hack.esp.draw();
//    //hack.aimbot.logic();
//
//    forward_to_original_sdl_swap_window()
//}
