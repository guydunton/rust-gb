lazy_static! {
    pub static ref DICTIONARY: Vec<(u8, Vec<&'static str>)> = vec![
        (0x00, "NOP"),
        (0x01, "LD16 BC d16"),
        (0x04, "INC B"),
        (0x05, "DEC B"),
        (0x06, "LD8 B d8"),
        (0x08, "LD16 (a16) SP"),
        (0x0B, "DEC BC"),
        (0x0C, "INC C"),
        (0x0D, "DEC C"),
        (0x0E, "LD8 C d8"),
        (0x11, "LD16 DE d16"),
        (0x13, "INC DE"),
        (0x15, "DEC D"),
        (0x16, "LD8 D d8"),
        (0x17, "RLA"),
        (0x18, "JP r8"),
        (0x1A, "LD8 A (DE)"),
        (0x1D, "DEC E"),
        (0x1E, "LD8 E d8"),
        (0x20, "JP NZ r8"),
        (0x21, "LD16 HL d16"),
        (0x22, "LD8 (HL+) A"),
        (0x23, "INC HL"),
        (0x24, "INC H"),
        (0x28, "JP Z r8"),
        (0x2A, "LD8 A (HL+)"),
        (0x2E, "LD8 L d8"),
        (0x31, "LD16 SP d16"),
        (0x32, "LD8 (HL-) A"),
        (0x36, "LD8 (HL) d8"),
        (0x3D, "DEC A"),
        (0x3E, "LD8 A d8"),
        (0x40, "LD8 B B"),
        (0x40, "LD8 B B"),
        (0x41, "LD8 B C"),
        (0x42, "LD8 B D"),
        (0x43, "LD8 B E"),
        (0x44, "LD8 B H"),
        (0x45, "LD8 B L"),
        (0x4F, "LD8 C A"),
        (0x57, "LD8 D A"),
        (0x67, "LD8 H A"),
        (0x77, "LD8 (HL) A"),
        (0x78, "LD8 A B"),
        (0x7B, "LD8 A E"),
        (0x7C, "LD8 A H"),
        (0x7D, "LD8 A L"),
        (0x86, "ADD A (HL)"),
        (0x90, "SUB B"),
        (0xAF, "XOR A"),
        (0xB1, "OR C"),
        (0xBE, "CP (HL)"),
        (0xC1, "POP BC"),
        (0xC3, "JP a16"),
        (0xC5, "PUSH BC"),
        (0xC9, "RET"),
        (0xCD, "CALL a16"),
        (0xE0, "LD8 (a8) A"),
        (0xE2, "LD8 (C) A"),
        (0xEA, "LD8 (a16) A"),
        (0xF0, "LD8 A (a8)"),
        (0xF9, "LD16 SP HL"),
        (0xF3, "NOP"),
        (0xF8, "LD16 HL SP+r8"),
        (0xFE, "CP d8"),
    ]
    .iter()
    .map(|(i, s)| (*i, s.split(' ').collect::<Vec<&'static str>>()))
    .collect();
}
