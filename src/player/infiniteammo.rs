use crate::offsets;
use crate::process;
use crate::ProcMem;

/**
 * works by patching the instruction that subtracts the number of
 * shot rounds from the current mag with a NOP
 */
pub struct InfiniteAmmo {
    address_to_patch: usize,
    enabled: bool,
    saved_instr: Option<u32>,

    // we use procmem for memory writes via /proc/mem
    // bypassing write protection on executable pages
    mem: ProcMem,
}

impl InfiniteAmmo {
    pub fn default() -> Self {
        InfiniteAmmo {
            address_to_patch: process::target::resolve_base_address().unwrap()
                + offsets::AMMO_PATCH,
            enabled: false,
            saved_instr: None,
            mem: ProcMem::init(),
        }
    }

    pub fn enable(&mut self) {
        if self.enabled {
            return;
        }

        println!("address to patch: {:#X}", self.address_to_patch);

        // if first time patching, save original instructions
        if self.saved_instr.is_none() {
            self.saved_instr = Some(self.mem.read(self.address_to_patch));
        }

        // patch with 3 bytes of NOPs (1*16B + 1*8B)
        self.mem.write(self.address_to_patch, 0x90_90 as u16);
        self.mem
            .write(self.address_to_patch + 2 as usize, 0x90 as u8);

        println!("infinite ammo enabled");

        self.enabled = true;
    }

    pub fn disable(&mut self) {
        if !self.enabled {
            return;
        }

        // should never happen
        if self.saved_instr.is_none() {
            panic!("tried to disable infinite ammo when already disabled");
        }

        // write back original bytes
        self.mem
            .write(self.address_to_patch, self.saved_instr.unwrap());

        self.enabled = false;
    }

    pub fn toggle(&mut self) {
        if self.enabled {
            self.disable();
        } else {
            self.enable();
        }
    }
}
