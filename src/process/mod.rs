/*
 * Finds the assault cube process and returns information about it: base of the different sections, pid etc.
 * This module can also inject an arbitrary SO file into a a process as a new thread and run a specified function from it.
 */

// re-exporting modules for shorter imports
pub mod helpers;
pub use self::helpers::*;
mod instantion;
pub mod memory;
pub use self::memory::*;
pub mod modules;
pub use self::modules::*;

use std::collections::HashMap;

// represents a loaded shared object
#[derive(Clone)]
pub struct Module {
    name: String,
    pub file: String,
    pub base_addr: usize,
    size: Option<usize>,
}

#[derive(Clone)]
pub struct Process {
    pub pid: usize,
    proc_dir: String,
    pub exe: String,
    is_internal: bool,
}

#[derive(Debug)]
pub enum ProcessErrors {
    // failure to interact with /proc (linux only)
    ProcDirFailure,

    // e.g. process is not running or exited
    ProcInvalid,

    // pid or exe name could not be linked to a valid process
    NotFound,

    // insufficient perms to access target process
    Permissions,

    // failed to open a file backing a module
    ModuleFileErr,
}

impl Process {
    pub fn get_current() -> Result<Self, ProcessErrors> {
        instantion::from_current()
    }

    pub fn get_module(&self, module_name: &str) -> Result<Module, ProcessErrors> {
        modules::get_module(self, module_name)
    }

    pub fn get_all_modules(&self) -> Result<HashMap<String, Module>, ProcessErrors> {
        modules::parse_modules(self)
    }
}
