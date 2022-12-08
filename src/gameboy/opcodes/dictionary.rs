lazy_static! {
    pub static ref DICTIONARY: Vec<(u8, Vec<&'static str>)> = vec![
        (0x00, "NOP"),
        (0x01, "LD16 BC d16"),
        (0x02, "LD8 (BC) A"),
        (0x03, "INC BC"),
        (0x04, "INC B"),
        (0x05, "DEC B"),
        (0x06, "LD8 B d8"),
        (0x08, "LD16 (a16) SP"),
        (0x09, "ADD16 HL BC"),
        (0x0A, "LD8 A (BC)"),
        (0x0B, "DEC BC"),
        (0x0C, "INC C"),
        (0x0D, "DEC C"),
        (0x0E, "LD8 C d8"),
        (0x11, "LD16 DE d16"),
        (0x12, "LD8 (DE) A"),
        (0x13, "INC DE"),
        (0x14, "INC D"),
        (0x15, "DEC D"),
        (0x16, "LD8 D d8"),
        (0x17, "RLA"),
        (0x18, "JP r8"),
        (0x19, "ADD16 HL DE"),
        (0x1A, "LD8 A (DE)"),
        (0x1B, "DEC DE"),
        (0x1C, "INC E"),
        (0x1D, "DEC E"),
        (0x1E, "LD8 E d8"),
        (0x20, "JP NZ r8"),
        (0x21, "LD16 HL d16"),
        (0x22, "LD8 (HL+) A"),
        (0x23, "INC HL"),
        (0x24, "INC H"),
        (0x25, "DEC H"),
        (0x26, "LD8 H d8"),
        (0x28, "JP Z r8"),
        (0x29, "ADD16 HL HL"),
        (0x2A, "LD8 A (HL+)"),
        (0x2B, "DEC HL"),
        (0x2C, "INC L"),
        (0x2D, "DEC L"),
        (0x2E, "LD8 L d8"),
        (0x2F, "CPL"),
        (0x31, "LD16 SP d16"),
        (0x32, "LD8 (HL-) A"),
        (0x33, "INC SP"),
        (0x34, "INC (HL)"),
        (0x35, "DEC (HL)"),
        (0x36, "LD8 (HL) d8"),
        (0x37, "SCF"),
        (0x39, "ADD16 HL SP"),
        (0x3A, "LD8 A (HL-)"),
        (0x3B, "DEC SP"),
        (0x3C, "INC A"),
        (0x3D, "DEC A"),
        (0x3E, "LD8 A d8"),
        (0x40, "LD8 B B"),
        (0x40, "LD8 B B"),
        (0x41, "LD8 B C"),
        (0x42, "LD8 B D"),
        (0x43, "LD8 B E"),
        (0x44, "LD8 B H"),
        (0x45, "LD8 B L"),
        (0x46, "LD8 B (HL)"),
        (0x47, "LD8 B A"),
        (0x48, "LD8 C B"),
        (0x49, "LD8 C C"),
        (0x4A, "LD8 C D"),
        (0x4B, "LD8 C E"),
        (0x4C, "LD8 C H"),
        (0x4D, "LD8 C L"),
        (0x4E, "LD8 C (HL)"),
        (0x4F, "LD8 C A"),
        (0x50, "LD8 D B"),
        (0x51, "LD8 D C"),
        (0x52, "LD8 D D"),
        (0x53, "LD8 D E"),
        (0x54, "LD8 D H"),
        (0x55, "LD8 D L"),
        (0x56, "LD8 D (HL)"),
        (0x57, "LD8 D A"),
        (0x58, "LD8 E B"),
        (0x59, "LD8 E C"),
        (0x5A, "LD8 E D"),
        (0x5B, "LD8 E E"),
        (0x5C, "LD8 E H"),
        (0x5D, "LD8 E L"),
        (0x5E, "LD8 E (HL)"),
        (0x5F, "LD8 E A"),
        (0x60, "LD8 H B"),
        (0x61, "LD8 H C"),
        (0x62, "LD8 H D"),
        (0x63, "LD8 H E"),
        (0x64, "LD8 H H"),
        (0x65, "LD8 H L"),
        (0x66, "LD8 H (HL)"),
        (0x67, "LD8 H A"),
        (0x68, "LD8 L B"),
        (0x69, "LD8 L C"),
        (0x6A, "LD8 L D"),
        (0x6B, "LD8 L E"),
        (0x6C, "LD8 L H"),
        (0x6D, "LD8 L L"),
        (0x6E, "LD8 L (HL)"),
        (0x6F, "LD8 L A"),
        (0x70, "LD8 (HL) B"),
        (0x71, "LD8 (HL) C"),
        (0x72, "LD8 (HL) D"),
        (0x73, "LD8 (HL) E"),
        (0x74, "LD8 (HL) H"),
        (0x75, "LD8 (HL) L"),
        (0x77, "LD8 (HL) A"),
        (0x78, "LD8 A B"),
        (0x79, "LD8 A C"),
        (0x7A, "LD8 A D"),
        (0x7B, "LD8 A E"),
        (0x7C, "LD8 A H"),
        (0x7D, "LD8 A L"),
        (0x7E, "LD8 A (HL)"),
        (0x7F, "LD8 A A"),
        (0x80, "ADD A B"),
        (0x81, "ADD A C"),
        (0x82, "ADD A D"),
        (0x83, "ADD A E"),
        (0x84, "ADD A H"),
        (0x85, "ADD A L"),
        (0x86, "ADD A (HL)"),
        (0x87, "ADD A A"),
        (0x90, "SUB B"),
        (0x91, "SUB C"),
        (0x92, "SUB D"),
        (0x93, "SUB E"),
        (0x94, "SUB H"),
        (0x95, "SUB L"),
        (0x97, "SUB A"),
        (0xA0, "AND B"),
        (0xA1, "AND C"),
        (0xA2, "AND D"),
        (0xA3, "AND E"),
        (0xA4, "AND H"),
        (0xA5, "AND L"),
        (0xA6, "AND (HL)"),
        (0xA7, "AND A"),
        (0xA8, "XOR B"),
        (0xA9, "XOR C"),
        (0xAA, "XOR D"),
        (0xAB, "XOR E"),
        (0xAC, "XOR H"),
        (0xAD, "XOR L"),
        (0xAF, "XOR A"),
        (0xB0, "OR B"),
        (0xB1, "OR C"),
        (0xB2, "OR D"),
        (0xB3, "OR E"),
        (0xB4, "OR H"),
        (0xB5, "OR L"),
        (0xB7, "OR A"),
        (0xBE, "CP (HL)"),
        (0xC0, "RET NZ"),
        (0xC1, "POP BC"),
        (0xC3, "JP a16"),
        (0xC5, "PUSH BC"),
        (0xC6, "ADD A d8"),
        (0xC7, "RST 00H"),
        (0xC8, "RET Z"),
        (0xC9, "RET"),
        (0xCD, "CALL a16"),
        (0xCF, "RST 08H"),
        (0xD1, "POP DE"),
        (0xD5, "PUSH DE"),
        (0xD7, "RST 10H"),
        (0xD9, "RETI"),
        (0xDF, "RST 18H"),
        (0xE0, "LD8 (a8) A"),
        (0xE1, "POP HL"),
        (0xE2, "LD8 (C) A"),
        (0xE5, "PUSH HL"),
        (0xE6, "AND d8"),
        (0xE7, "RST 20H"),
        (0xE9, "JP (HL)"),
        (0xEA, "LD8 (a16) A"),
        (0xEF, "RST 28H"),
        (0xF0, "LD8 A (a8)"),
        (0xF1, "POP AF"),
        (0xF2, "LD8 A (C)"),
        (0xF7, "RST 30H"),
        (0xF9, "LD16 SP HL"),
        (0xFA, "LD8 A (a16)"),
        (0xF3, "DI"),
        (0xF5, "PUSH AF"),
        (0xF8, "LD16 HL SP+r8"),
        (0xFB, "EI"),
        (0xFE, "CP d8"),
        (0xFF, "RST 38H"),
    ]
    .iter()
    .map(|(i, s)| (*i, s.split(' ').collect::<Vec<&'static str>>()))
    .collect();
}
