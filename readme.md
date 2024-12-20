
## What ?

`ac_internal` is a Rust-based dynamic library for Assault Cube, providing features such as ESP boxes, godmode and infinite ammo. We load our logic into the target process memory region (hence "internal") with `LD_PRELOAD`, enabling the real-time modification of game behavior through direct memory manipulation.

## Why ?

To deepen my understanding of:

  - Memory allocation, structure, and interaction.
  - Function Hooking: Intercepting and detouring game logic at the assembly level.
  - Low-level Programming: Writing shellcode in x64 assembly and ensuring correct register and memory management.
  - Interoperability: Combining Rust with system libraries like SDL2 and OpenGL.
  - Real-time Graphics Rendering: Utilizing OpenGL to draw ESP elements dynamically within the game window.

### Features
1. Direct Memory Access

  - Inject into the game's memory space using `LD_PRELOAD`.
  - Resolve function addresses using `libloading` and mutate memory with Rust’s API.

2. Godmode

  - Detour original game instructions to shellcode
  - Enable nullification of damage to user's character and one-hit-kills of enemies.

3. Infinite Ammo

  - Overwrite assembly instructions responsible for ammo depletion with NOPs.

4. ESP

  - Leverage OpenGL bindings to add boxes around player entities to rendered frames.

# Getting Started

## Prerequisites

- **Rust and Cargo**: Install from [rustup.rs](https://rustup.rs/).
- **Assault Cube Game**: Install from https://assault.cubers.net/.

---

## Steps

### 1. Clone the Repository

```sh
git clone https://github.com/Hotz99/ac_internal.git
cd ac_internal
```

### 2. Build the Library

To compile the library in debug mode with debug symbols:

`cargo build`

The compiled library will be in `target/debug/`.


### Debugging in VS Code

Using the `lldb` debugger, an example `launch.json` configuration is:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug AssaultCube",
            "program": "<absolute_path_to_assault_cube_executable>/linux_64_client",
            "args": [],
            "cwd": "<absolute_path_to_assault_cube_directory>",
            "env": {
                "LD_PRELOAD": "<absolute_path_to_compiled_library>/libac_internal.so"
            }
        }
    ]
}
```

### Manual Testing

Alternatively, you can manually launch the game with LD_PRELOAD to test the library:

`LD_PRELOAD=<absolute_path_to_compiled_library>/libac_internal.so <absolute_path_to_assault_cube_executable>/linux_64_client`

## Further Work

1. **Refactoring and Optimization**
   - Improve code structure for better understandability and maintainability.
   - Optimize memory access and rendering logic.

2. **Fix Outdated Memory Addresses**
    - Many addresses are outdated, from the game version the original repo was based on.
    - Reverse engineering with Cheat Engine takes time, even with the source code available.

3. **Error Handling and Stability**
   - Implement robust error handling.
   - Employ signature scanning to ensure compatibility across different versions of Assault Cube
   
4. **In-game menu with `Imgui`**

5. **Proper Documentation and Testing**

## Credits

This project builds upon the [ac_rhack](https://github.com/scannells/ac_rhack) repository. Thank you to the author for their foundational work.