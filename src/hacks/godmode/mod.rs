use crate::{offsets, process, utils};

/* godmode works by patching the instruction that writes to the health of a
 * player. we can't just add a NOP here, because all players would take not damage
 * hence we patch the original damage function instructions to jmp to a memory page containing
 * custom (assembled) instructions, a shellcode. then we apply damage to a player if is not
 * our local_player */

// original instructions = 19 bytes
// hook instructions must add up to 19 bytes
const HOOK_SIZE: usize = 19;

pub struct GodMode {
    addr_to_patch: usize,
    local_player_addr: usize,
    is_enabled: bool,
    one_hit_kills: bool,
    original_instructions: Option<[u8; HOOK_SIZE]>,
    proc_mem: process::ProcMem,
    // address of rwx page containing shellcode
    page_address: Option<*mut std::ffi::c_void>,
    assembled_patch: Option<Vec<u8>>,
}

impl GodMode {
    pub fn new(game_base_addr: usize, local_player_addr: usize) -> Self {
        GodMode {
            addr_to_patch: game_base_addr + offsets::DAMAGE_PATCH,
            local_player_addr,
            is_enabled: false,
            one_hit_kills: false,
            original_instructions: None,
            proc_mem: process::ProcMem::init(),
            page_address: None,
            assembled_patch: None,
        }
    }

    pub fn enable(&mut self) {
        if self.is_enabled {
            return;
        }

        if self.original_instructions.is_none() {
            /*
             * 1. allocate rwx memory page for our shellcode, at any address
             * 2. patch damage function: instead of doing damage, jump to the page containing
             *    our shellcode
             * 3. original 4 instructions:
             *   1.  sub     eax,edx                   # patching over it bc we need space
             *   2.  mov     [r14+0x104],eax           # patching over it bc we need space
             *   3.  sub     r12d,esi                  # r12d will contain the damage
             *   4.  sub     [r14+0x100],r12d          # instruction that applies damage, r14 holds a player struct address
             * 4. bytecode of the above instructions:
             *   1.  29 D0
             *   2.  41 89 86 04 01 00 00
             *   3.  41 29 F4
             *   4.  45 29 A6 00 01 00 00
             *
             *   total = 19 bytes
             *
             *   the shellcode we jump to must include these instructions and
             *   set and return the registers to their original state
             */

            self.page_address = Some(utils::shellcode::get_executable_map(4096));

            println!("address_to_patch: 0x{:x}", self.addr_to_patch);

            let return_addr = self.addr_to_patch + HOOK_SIZE;

            let shellcode_instructions = format!(
                r#"BITS 64              ; NASM setting 
                pop rax                 ; clear shellcode address from stack after jump
                ; replicate original instructions
                sub eax,edx
                ; replicate original instructions
                mov [r14+0x104],eax
                ; move local player base address into rax
                mov rax,0x{0:x}
                ; r14 contains the address of damage target (player struct instance)
                cmp rax,r14
                ; if target is local_player, jmp to ret without applying damage
                je exit
                ; if target is another player, enable 1-hit kills by applying 1000 damage
                ; damage will be in r12d after r12d - esi 
                sub r12d,esi
                mov r12d,1000
                ; apply damage
                sub [r14+0x100],r12d

                exit:
                ; push [patch_address + hook_size] to stack 
                mov rax,0x{1:x}
                push rax
                ; pop return address from stack and jump to there
                ret
                "#,
                self.local_player_addr, return_addr
            );

            let shellcode = utils::shellcode::nasm_assemble(shellcode_instructions);

            self.proc_mem
                .write_n(self.page_address.unwrap() as usize, &shellcode);

            // original instructions total 19 bytes
            // patch instructions total 14 bytes
            // so add 5 NOPs
            let patch_instructions = format!(
                r#"
                ; NASM setting
                BITS 64;              
                ; save rax before jumping 
                push rax                  
                ; move patch shellcode page address into rax
                mov rax, 0x{:x}          
                ; detour to shellcode page
                jmp rax                  
                ; clear stack after returning from shellcode
                pop rax                  
                NOP                      
                NOP                      
                NOP                      
                NOP                      
                NOP                      
                "#,
                self.page_address.unwrap() as usize
            );

            self.assembled_patch = Some(utils::shellcode::nasm_assemble(patch_instructions));

            let mut original_instructions = [0; HOOK_SIZE];
            for i in 0..original_instructions.len() {
                original_instructions[i] = self.proc_mem.read(self.addr_to_patch + i);
            }

            self.original_instructions = Some(original_instructions);
        }

        self.proc_mem
            .write_n(self.addr_to_patch, &self.assembled_patch.as_ref().unwrap());

        println!(
            "[AC_INTERNAL] patched address at 0x{:x} with len {}",
            self.addr_to_patch,
            &self.assembled_patch.as_ref().unwrap().len()
        );

        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        if !self.is_enabled {
            return;
        }

        self.proc_mem
            .write_n(self.addr_to_patch, &self.original_instructions.unwrap());

        self.is_enabled = false;
    }

    pub fn toggle(&mut self) -> bool {
        if self.is_enabled {
            self.disable();
        } else {
            self.enable();
        }

        self.is_enabled
    }
}
