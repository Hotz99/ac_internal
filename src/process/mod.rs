use std::collections::HashMap;

mod helpers;
/**
 * Finds the assault cube process and returns information about it: base of the different sections, pid etc.
 * This module can also inject an arbitrary SO file into a a process as a new thread and run a specified function from it.
 */
mod instantion;
mod memory;
mod modules;
pub mod target;

// export all public symbols of the sub modules
pub use memory::*;
pub use modules::*;
//pub use injection::*;

/// represents a loaded binary or shared object file (e.g. the binary itself or libc)
#[derive(Clone)]
pub struct Module {
    name: String,
    pub file: String,
    pub base: usize,
    size: Option<usize>,
}

#[derive(Clone)]
pub struct Process {
    pub pid: usize,
    proc_dir: String,
    pub exe: String,
    is_internal: bool,
}

/// Indicates the reason for a failure on a process operation
#[derive(Debug)]
pub enum ProcessErrors {
    /// On Linux systems, this indicates a failure to interact with /proc
    ProcDirFailure,

    /// A process became invalid (e.g. it exited)
    ProcInvalid,

    /// When a PID or exe name could not be linked to a valid process
    NotFound,

    /// Permissions are insufficient to get access to the target process
    Permissions,

    /// Failed to open a file backing a module
    ModuleFileErr,
}

/// Main struct
impl Process {
    pub fn current() -> Result<Self, ProcessErrors> {
        instantion::from_current()
    }

    pub fn module(&self, module_name: &str) -> Result<Module, ProcessErrors> {
        modules::get_module(self, module_name)
    }

    pub fn modules(&self) -> Result<HashMap<String, Module>, ProcessErrors> {
        modules::parse_modules(self)
    }
}
