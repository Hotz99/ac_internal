use crate::{offsets, process, utils};

pub struct InfiniteAmmo {
    addr_to_patch: usize,
    is_enabled: bool,
    original_instructions: Option<u32>,

    proc_mem: process::ProcMem,
}

impl InfiniteAmmo {
    pub fn default() -> Self {
        InfiniteAmmo {
            addr_to_patch: utils::game_base::resolve_addr() + offsets::AMMO_DECREASE_FN,
            is_enabled: false,
            original_instructions: None,
            proc_mem: process::ProcMem::init(),
        }
    }

    pub fn enable(&mut self) {
        if self.is_enabled {
            return;
        }

        // if first time patching, save original instructions
        if self.original_instructions.is_none() {
            self.original_instructions = Some(self.proc_mem.read(self.addr_to_patch));
        }

        // patch with 3 bytes of NOPs (1*16B + 1*8B)
        self.proc_mem.write(self.addr_to_patch, 0x90_90 as u16);
        self.proc_mem
            .write(self.addr_to_patch + 2 as usize, 0x90 as u8);

        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        if !self.is_enabled {
            return;
        }

        // should never happen
        if self.original_instructions.is_none() {
            panic!("tried to disable infinite ammo when already disabled");
        }

        // write back original bytes
        self.proc_mem
            .write(self.addr_to_patch, self.original_instructions.unwrap());

        self.is_enabled = false;
    }

    // TODO make toggle() a trait
    pub fn toggle(&mut self) -> bool {
        if self.is_enabled {
            self.disable();
        } else {
            self.enable();
        }

        self.is_enabled
    }
}
