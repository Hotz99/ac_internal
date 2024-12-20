use std::io::prelude::*;

// returns pointer to allocated executable page for our shellcode
pub fn get_executable_map(map_size: usize) -> *mut std::ffi::c_void {
    let map_size = std::num::NonZeroUsize::new(map_size).expect("map size must be non-zero");

    let mut prot_flags = nix::sys::mman::ProtFlags::empty();
    prot_flags.insert(nix::sys::mman::ProtFlags::PROT_READ);
    prot_flags.insert(nix::sys::mman::ProtFlags::PROT_EXEC);

    let mut map_flags = nix::sys::mman::MapFlags::empty();
    map_flags.insert(nix::sys::mman::MapFlags::MAP_PRIVATE);
    map_flags.insert(nix::sys::mman::MapFlags::MAP_ANON);

    let rw_page = unsafe {
        // shelcode is not associated with a file, hence mmap_anonymous()
        nix::sys::mman::mmap_anonymous(
            // allocate at random address
            None, map_size, prot_flags, map_flags,
        )
        .expect("failed to allocate executable map for shellcode")
    };

    rw_page.as_ptr()
}

// input must be x86_64 assembly
pub fn nasm_assemble(shellcode: String) -> Vec<u8> {
    let mut asm_file =
        std::fs::File::create("/tmp/ac_hack_asm.S").expect("Failed to write to /tmp");
    asm_file
        .write_all(shellcode.as_bytes())
        .expect("failed to write assembly to /tmp file");

    // assemble with nasm
    std::process::Command::new("nasm")
        .arg("-f")
        .arg("bin")
        .arg("/tmp/ac_hack_asm.S")
        .arg("-o")
        .arg("/tmp/ac_hack_asm")
        .status()
        .expect("this program requires NASM to generate shellcode but it is not installed");

    std::fs::remove_file(std::path::Path::new("/tmp/ac_hack_asm.S"))
        .expect("failed to delete tmp shellcode file");

    // read the resulting opcodes into a u8 vec - start with a 4096byte buffer
    let mut asm_file = std::fs::File::open("/tmp/ac_hack_asm")
        .expect("failed to open assembled shellcode file for reading");
    let size = asm_file.metadata().unwrap().len();

    let mut assembled_code: Vec<u8> = vec![0; size as usize];
    asm_file
        .read(&mut assembled_code)
        .expect("failed to read assembly dump");

    std::fs::remove_file(std::path::Path::new("/tmp/ac_hack_asm"))
        .expect("failed to delete tmp shellcode file");
    assembled_code
}
