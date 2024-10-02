use core::ffi::c_void;

use crate::{Player, ProcMem};

use crate::util::game_base;
use crate::{gen_shellcode, get_executable_map};

/* Enabling gode mode works by patching the instruction that writes to the health of a
 * player. We can't just add a NOP here, as that would mean no one can die anymore.
 * For this reason we must allocate an executable page where we write code that will
 * check if the health address that is supposed to be written to is the health address
 * of the current player. If it is, we just NOP
 * If it isn't, we set the damage to be 100 and proceed to subtract the damage
 * the instruction that subtracts the damage is
 * sub dword [rbx+0x110], ebp
 * We will patch this instruction and some instructions around it to jump to that page and then
 * restore registers
 */
const DAMAGE_FUNCTION_OFFSET: usize = 0x2fd10;
// hook = detour logic (jmp) + patch logic (shellcode in page)
// original instructions = 19 bytes
// hook instructions must add up to 19 bytes
const HOOK_SIZE: usize = 19;

pub struct GodMode {
    // address of first instruction to patch
    address_to_patch: usize,
    is_enabled: bool,
    original_instructions: Option<[u8; HOOK_SIZE]>,

    // ProcMem writes via /proc/mem
    // bypassing write protection on executable pages
    mem: ProcMem,

    // reference to executable page containing the shellcode
    page: Option<*mut c_void>,

    // local player base address used in the shellcode
    player_base: usize,

    // shellcode used in damage function detour
    patch_shellcode: Option<Vec<u8>>,
}

impl GodMode {
    pub fn new() -> Self {
        GodMode {
            address_to_patch: game_base() + DAMAGE_FUNCTION_OFFSET,
            is_enabled: false,
            original_instructions: None,
            mem: ProcMem::init(),
            page: None,
            player_base: Player::local_player().base,
            patch_shellcode: None,
        }
    }

    pub fn enable(&mut self) {
        // nothing to do
        if self.is_enabled {
            return;
        }

        // If this is the first time patching, make sure to have saved the instruction before
        // so that we can restore the code
        if self.original_instructions.is_none() {
            /* we need to allocate a rwx page for our payload hook
             * 1. allocate a r-x map at any address
             * 2. patch the damage function: instead of doing damage, jump to the page containing
             *    our patch shellcode
             * 3. we will replace 4 instructions to make space for our hook:
             *   1.  sub     eax, edx                 # needed for space
             *   2.  mov     [r14+0x104], eax   # this instruction has nothing to do with our code but is needed for space
             *   3.  sub     r12d, esi                 # r12d will contain the damage
             *   4.  sub     [r14+0x100], r12d   # this is the instruction that does damage, r14 is local player address
             * 4. bytecode of the above instructions:
             *   1.  29 D0
             *   2.  41 89 86 04 01 00 00
             *   3.  45 29 F4
             *   4.  45 29 A6 00 01 00 00
             *
             *   total = 19 bytes
             *
             *    the patch shellcode we will jump to must include these instructions and
             *    set and return the registers to their original state
             */
            self.page = Some(get_executable_map(4096));

            let return_address = self.address_to_patch + HOOK_SIZE;

            let patch_instructions = format!(
                r#"BITS 64               ; NASM stuff
    ; replicate original instructions
    sub eax, edx
    ; replicate original instructions
    mov [r14+0x104], eax
    ; move local player base address into rax
    mov rax,  0x{0:x}
    ; rbx contains the address of damage target player struct
    cmp rax, rbx
    ; if target equal local player, turn this into a NOP by jumping to ret
    jz exit
    ; if target is another player, enable 1-hit kills by applying damage to over 9000
    ; damage will be in ebp after ebp - edx
    sub ebp, edx
    ; save rbp in case the value is needed later
    push rbp
    mov ebp, 9001
    ; apply damage
    sub [rbx+0x100],ebp
    pop rbp
    \
    ; replicate original instructions
    sub r12d, esi
    ; replicate original instructions
    sub [r14+0x100], r12d 

    exit:
    ; push [patch_address + hook_size] to stack 
    mov rax, 0x{1:x}
    push rax
    ; pop return address from stack and jump to there
    ret
    "#,
                self.player_base, return_address
            );

            println!("patch return address: 0x{:x}", return_address);

            let shellcode = gen_shellcode(patch_instructions);

            self.mem.write_n(self.page.unwrap() as usize, &shellcode);

            // original instructions total 19 bytes
            // patch instructions total 14 bytes
            // so add 5 NOPs to offset the difference
            let patch_instructions = format!(
                r#"BITS 64;              ; NASM stuff
                push rax                 ; save rax in the stack
                mov rax, 0x{:x}          ; move patch shellcode page address into rax
                jmp rax                  ; detour to shellcode page
                pop rax                  ; restore rax after patch ret
                NOP                      ; NOP for padding
                NOP                      ; NOP for padding
                NOP                      ; NOP for padding
                NOP                      ; NOP for padding
                NOP                      ; NOP for padding"#,
                self.page.unwrap() as usize
            );

            // assemble the detour instructions
            self.patch_shellcode = Some(gen_shellcode(patch_instructions));

            // before overwriting the patch address, we need to save it
            let mut original_instructions = [0; HOOK_SIZE];
            for i in 0..original_instructions.len() {
                original_instructions[i] = self.mem.read(self.address_to_patch);
            }

            self.original_instructions = Some(original_instructions);
        }

        self.mem.write_n(
            self.address_to_patch,
            &self.patch_shellcode.as_ref().unwrap(),
        );

        println!(
            "patched address at 0x{:x} with len {}",
            self.address_to_patch,
            &self.patch_shellcode.as_ref().unwrap().len()
        );

        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        // nothing to do if this patch is already enabled
        if !self.is_enabled {
            return;
        }

        // make sure the code can't accidentally disable without having
        // read the original instructions before
        if self.original_instructions.is_none() {
            panic!("Tried to disable godmode without ever having enabled it");
        }

        // simply write back the original bytes
        self.mem
            .write_n(self.address_to_patch, &self.original_instructions.unwrap());

        self.is_enabled = false;
    }

    pub fn toggle(&mut self) {
        if self.is_enabled {
            self.disable();
        } else {
            self.enable();
        }
    }
}
