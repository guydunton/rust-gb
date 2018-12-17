use super::read_write_register::ReadWriteRegister;
use super::register::RegisterLabel8;

#[derive(Copy, Clone, Debug)]
pub enum Flags {
    Z, // Zero
    N, // Subtract
    H, // Half carry
    C, // Carry
}

#[allow(dead_code)]
fn get_flag(register: u8, flag: Flags) -> bool {
    match flag {
        Flags::Z => (0b1000_0000 & register) != 0,
        Flags::N => (0b0100_0000 & register) != 0,
        Flags::H => (0b0010_0000 & register) != 0,
        Flags::C => (0b0001_0000 & register) != 0,
    }
}

#[allow(dead_code)]
fn set_flag(register: u8, flag: Flags, on: bool) -> u8 {
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

pub fn read_flag<T: ReadWriteRegister>(cpu: &dyn ReadWriteRegister, flag: Flags) -> bool {
    get_flag(cpu.read_8_bits(RegisterLabel8::F), flag)
}

pub fn write_flag<T: ReadWriteRegister>(cpu: &mut dyn ReadWriteRegister, flag: Flags, on: bool) {
    let flags = cpu.read_8_bits(RegisterLabel8::F);
    cpu.write_8_bits(RegisterLabel8::F, set_flag(flags, flag, on));
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
