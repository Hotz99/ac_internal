use std::io::BufRead;

pub fn resolve_addr() -> usize {
    // our logic is injected into the target process
    // hence id() returns target process id
    let target_pid = std::process::id();
    // directory containing memory maps of the target process
    let maps_path = format!("/proc/{}/maps", target_pid);

    if let Ok(file) = std::fs::File::open(maps_path) {
        let reader = std::io::BufReader::new(file);
        if let Some(Ok(line)) = reader.lines().next() {
            // first line has format [address_range perms offset dev inode pathname]
            if let Some(address_range) = line.split_whitespace().next() {
                if let Some(base_address) = address_range.split('-').next() {
                    // parse base address as a hex
                    return usize::from_str_radix(base_address, 16).unwrap();
                }
            }
        }
    }

    0x0
}
