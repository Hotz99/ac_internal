mod memdata;
use self::memdata::MemData;

use crate::process;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

// wrapper for reading/writing memory through /proc/mem
// used for bypassing rwx protections
pub struct ProcMem {
    handle: File,
}

impl ProcMem {
    pub fn init() -> Self {
        let process = process::Process::get_current().unwrap();
        let mempath = format!("{}/mem", &process.proc_dir);
        let mempath = Path::new(&mempath);
        let memhandle = OpenOptions::new()
            .read(true)
            .write(true)
            .open(mempath)
            .expect("failed to open /proc/self/mem for memory operations");

        ProcMem { handle: memhandle }
    }

    pub fn write<T: memdata::MemData>(&mut self, addr: usize, data: T) {
        self.handle
            .seek(SeekFrom::Start(addr as u64))
            .expect("failed to seek /proc/self/mem file");

        self.handle
            .write(&data.get_vec())
            .expect("failed write to /proc/self/mem file");
    }

    pub fn read<T: memdata::MemData + Copy>(&mut self, addr: usize) -> T {
        self.handle
            .seek(SeekFrom::Start(addr as u64))
            .expect("failed to seek /proc/self/mem file");

        let mut _buf = T::make_buf();
        self.handle
            .read(&mut _buf)
            .expect("failed to read from /proc/self/mem file");

        T::from_vec(&_buf)
    }

    // akin to memcpy() for larger buffers
    pub fn write_n(&mut self, addr: usize, data: &[u8]) {
        let mut rest = data.len();
        let mut curr = 0;
        while rest != 0 {
            let size = {
                if rest % 8 == 0 {
                    let bytes = u64::from_vec(&Vec::from(&data[curr..curr + 8]));
                    self.write(addr + curr, bytes);
                    8
                } else if rest % 4 == 0 {
                    let bytes = u32::from_vec(&Vec::from(&data[curr..curr + 4]));
                    self.write(addr + curr, bytes);
                    4
                } else if rest % 2 == 0 {
                    let bytes = u16::from_vec(&Vec::from(&data[curr..curr + 2]));
                    self.write(addr + curr, bytes);
                    2
                } else {
                    let bytes = data[curr];
                    self.write(addr + curr, bytes);
                    1
                }
            };

            rest -= size;
            curr += size;
        }
    }

    pub fn read_n(&mut self, addr: usize, size: usize) -> Vec<u8> {
        let mut ret = Vec::new();
        let mut rest = size;
        let mut curr = 0;
        while rest != 0 {
            let data = self.read::<u8>(addr + curr);
            ret.push(data);
            rest -= 1;
            curr += 1;
        }

        ret
    }
}

// wrapper for reading/writing dynamic data through pointers
#[derive(Clone)]
pub struct InternalMemory {}

impl InternalMemory {
    pub fn write<T: memdata::MemData>(addr: usize, data: T) {
        let ptr: *mut T = addr as *mut T;
        unsafe { *ptr = data };
    }

    pub fn read<T: memdata::MemData + Copy>(addr: usize) -> T {
        let ptr: *const T = addr as *const T;
        let ret: T = unsafe { *ptr };
        ret
    }
}

#[derive(Debug)]
pub enum MemoryError {
    ProcInvalid,
    InvalidTechnique,
}
