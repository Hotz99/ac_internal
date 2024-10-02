use crate::offsets;
use crate::util::game_base;
use crate::ProcMem;

/**
 * InfiniteAmmo works by patching the instruction that subtracts the number of
 * shot rounds from the current mag with a NOP
 */
pub struct InfiniteAmmo {
    address_to_patch: usize,
    enabled: bool,
    saved_instr: Option<u32>,

    // the reason we use procmem here is that memory writes via /proc/mem
    // bypass write protection on executable pages
    mem: ProcMem,
}

impl InfiniteAmmo {
    pub fn new() -> Self {
        InfiniteAmmo {
            address_to_patch: game_base() + offsets::AMMO_PATCH_OFFSET,
            enabled: false,
            saved_instr: None,
            mem: ProcMem::init(),
        }
    }

    pub fn enable(&mut self) {
        // nothing to do
        if self.enabled {
            return;
        }

        // If this is the first time patching, make sure to have saved the instruction before
        // so that we can restore the code
        if !self.saved_instr.is_some() {
            self.saved_instr = Some(self.mem.read(self.address_to_patch));
        }

        // patch the instruction with 3 bytes of NOPs (1x 16 bytes and 1x 8 byte write)
        self.mem.write(self.address_to_patch, 0x90_90 as u16);
        self.mem
            .write(self.address_to_patch + 2 as usize, 0x90 as u8);

        println!("Infinite ammo enabled");
        println!("Address to patch: {:#X}", self.address_to_patch);

        // keep a record that this hook is enabled
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        // nothing to do if this patch is already enabled
        if !self.enabled {
            return;
        }

        // make sure the code can't accidentally disable without having
        // read the original instructions before
        if !self.saved_instr.is_some() {
            panic!("Tried to disable infinite ammo without ever having enabled it");
        }

        // simply write back the original bytes
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
