use super::cb_opcodes::cb_code_to_opcode;

pub fn code_to_opcode(code: u8, program_counter: u16, program_code: &[u8]) -> Result<&str, String> {
    match code {
        0x00 => Ok("NOP"),
        0x04 => Ok("INC B"),
        0x05 => Ok("DEC B"),
        0x06 => Ok("LD8 B d8"),
        0x0C => Ok("INC C"),
        0x0D => Ok("DEC C"),
        0x0E => Ok("LD8 C d8"),
        0x11 => Ok("LD16 DE d16"),
        0x13 => Ok("INC DE"),
        0x17 => Ok("RLA"),
        0x18 => Ok("JR r8"),
        0x1A => Ok("LD8 A (DE)"),
        0x1D => Ok("DEC E"),
        0x1E => Ok("LD8 E d8"),
        0x20 => Ok("JR NZ r8"),
        0x21 => Ok("LD16 HL d16"),
        0x22 => Ok("LD8 (HL+) A"),
        0x23 => Ok("INC HL"),
        0x24 => Ok("INC H"),
        0x28 => Ok("JR Z r8"),
        0x2E => Ok("LD8 L d8"),
        0x31 => Ok("LD16 SP d16"),
        0x32 => Ok("LD8 (HL-) A"),
        0x3D => Ok("DEC A"),
        0x3E => Ok("LD8 A d8"),
        0x4F => Ok("LD8 C A"),
        0x57 => Ok("LD8 D A"),
        0x67 => Ok("LD8 H A"),
        0x77 => Ok("LD8 (HL) A"),
        0x7B => Ok("LD8 A E"),
        0xAF => Ok("XOR A"),
        0xC1 => Ok("POP BC"),
        0xC5 => Ok("PUSH BC"),
        0xC9 => Ok("RET"),
        0xCB => {
            // 0xCB is prefix and the next byte shows the actual instruction
            let cb_instruction = program_code[program_counter as usize + 1];
            return cb_code_to_opcode(cb_instruction);
        }
        0xCD => Ok("CALL a16"),
        0xE0 => Ok("LD8 (a8) A"),
        0xE2 => Ok("LD8 (C) A"),
        0xEA => Ok("LD8 (a16) A"),
        0xF0 => Ok("LD8 A (a8)"),
        0xFE => Ok("CP d8"),
        _ => Err(format!(
            "Unknown command {:#X} at address: {:#X}",
            code, program_counter
        )),
    }
}
