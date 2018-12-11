pub enum Flags {
    Z, // Zero
    N, // Subtract
    H, // Half carry
    C, // Carry
}

pub fn get_flag(register: u8, flag: Flags) -> bool {
    match flag {
        Flags::Z => (0b1000_0000 & register) != 0,
        Flags::N => (0b0100_0000 & register) != 0,
        Flags::H => (0b0010_0000 & register) != 0,
        Flags::C => (0b0001_0000 & register) != 0,
    }
}

pub fn set_flag(register: u8, flag: Flags, on: bool) -> u8 {
    match (flag, on) {
        (Flags::Z, false) => register & 0b0111_1111,
        (Flags::Z, true) => register | 0b1000_0000,
        (Flags::N, false) => register & 0b1011_1111,
        (Flags::N, true) => register | 0b0100_0000,
        (Flags::H, false) => register & 0b0101_1111,
        (Flags::H, true) => register | 0b0010_0000,
        (Flags::C, false) => register & 0b0110_1111,
        (Flags::C, true) => register | 0b0001_0000,
    }
}

#[test]
fn test_flag() {
    assert_eq!(get_flag(0b1000_0000, Flags::Z), true);
    assert_eq!(get_flag(0b0000_0000, Flags::Z), false);
    assert_eq!(get_flag(0b0100_0000, Flags::N), true);
    assert_eq!(get_flag(0b0000_0000, Flags::N), false);
    assert_eq!(get_flag(0b0010_0000, Flags::H), true);
    assert_eq!(get_flag(0b0000_0000, Flags::H), false);
    assert_eq!(get_flag(0b0001_0000, Flags::C), true);
    assert_eq!(get_flag(0b0000_0000, Flags::C), false);
}

#[test]
fn set_flag_test() {
    assert_eq!(set_flag(0b0000_0000, Flags::Z, true), 0b1000_0000);
    assert_eq!(set_flag(0b1000_0000, Flags::Z, false), 0b0000_0000);
    assert_eq!(set_flag(0b1000_0000, Flags::Z, true), 0b1000_0000);
    assert_eq!(set_flag(0b0000_0000, Flags::Z, false), 0b0000_0000);
}

// Possible tests for registers that can automatically put/push values to/from a cpu
// #[test]
// fn test_flag() {
//     let mut cpu = CPU::new();

//     cpu.write_8_bits(RegisterLabel8::F, 0b1000_0000);
//     assert_eq!(cpu.get_flag(Flags::Z), true);
//     cpu.write_8_bits(RegisterLabel8::F, 0b0000_0000);
//     assert_eq!(cpu.get_flag(Flags::Z), false);
//     cpu.write_8_bits(RegisterLabel8::F, 0b0100_0000);
//     assert_eq!(cpu.get_flag(Flags::N), true);
//     cpu.write_8_bits(RegisterLabel8::F, 0b0000_0000);
//     assert_eq!(cpu.get_flag(Flags::N), false);
//     cpu.write_8_bits(RegisterLabel8::F, 0b0010_0000);
//     assert_eq!(cpu.get_flag(Flags::H), true);
//     cpu.write_8_bits(RegisterLabel8::F, 0b0000_0000);
//     assert_eq!(cpu.get_flag(Flags::H), false);
//     cpu.write_8_bits(RegisterLabel8::F, 0b0001_0000);
//     assert_eq!(cpu.get_flag(Flags::C), true);
//     cpu.write_8_bits(RegisterLabel8::F, 0b0000_0000);
//     assert_eq!(cpu.get_flag(Flags::C), false);
// }

// #[test]
// fn set_flag_test() {
//     let mut cpu = CPU::new();

//     cpu.write_8_bits(RegisterLabel8::F, 0b0000_0000);
//     cpu.set_flag(Flags::Z, true);
//     assert_eq!(cpu.read_8_bits(RegisterLabel8::F), 0b1000_0000);

//     cpu.write_8_bits(RegisterLabel8::F, 0b1000_0000);
//     cpu.set_flag(Flags::Z, false);
//     assert_eq!(cpu.read_8_bits(RegisterLabel8::F), 0b0000_0000);

//     cpu.write_8_bits(RegisterLabel8::F, 0b1000_0000);
//     cpu.set_flag(Flags::Z, true);
//     assert_eq!(cpu.read_8_bits(RegisterLabel8::F), 0b1000_0000);

//     cpu.write_8_bits(RegisterLabel8::F, 0b1000_0000);
//     cpu.set_flag(Flags::Z, false);
//     assert_eq!(cpu.read_8_bits(RegisterLabel8::F), 0b0000_0000);
// }
