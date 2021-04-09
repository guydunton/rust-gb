use crate::{
    gameboy::{Flags, RegisterLabel16, RegisterLabel8},
    Gameboy,
};

fn add_fixture_gb<'a>(code: u8, a_val: u8, source_val: u8) -> Gameboy<'a> {
    let mut gb = Gameboy::new(vec![code]);
    gb.set_register_8(RegisterLabel8::A, a_val);
    gb.set_register_16(RegisterLabel16::HL, 0x4000);
    gb.set_memory_at(0x4000, source_val);
    gb
}

#[test]
fn add_hl_size_test() {
    let mut gb = add_fixture_gb(0x86, 1, 2);

    let cycles = gb.step_once();

    assert_eq!(cycles, 8);
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

    // The result is written back to A
    assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x03);

    // ADD A (HL) sets the N flag to 0
    assert!(!gb.get_flag(Flags::N));
    assert!(!gb.get_flag(Flags::Z));
}

#[test]
fn add_hl_sets_z_if_result_0() {
    let mut gb = add_fixture_gb(0x86, 0, 0);
    gb.step_once();

    assert!(gb.get_flag(Flags::Z));
}

#[test]
fn add_hl_h_flag_overflow_at_half_byte() {
    let mut gb = add_fixture_gb(0x86, 0b0000_1111, 1);
    gb.step_once();

    assert!(gb.get_flag(Flags::H));
}

#[test]
fn add_hl_c_flag_overflow_byte() {
    let mut gb = add_fixture_gb(0x86, 0b1111_1111, 1);
    gb.step_once();
    assert!(gb.get_flag(Flags::C));
}

// Reset the flags is they are already set
#[test]
fn add_hl_reset_flags_if_set() {
    let mut gb = add_fixture_gb(0x86, 1, 1);
    gb.set_flag(Flags::Z, true);
    gb.set_flag(Flags::H, true);
    gb.set_flag(Flags::N, true);
    gb.set_flag(Flags::C, true);
    gb.step_once();

    assert!(!gb.get_flag(Flags::Z));
    assert!(!gb.get_flag(Flags::H));
    assert!(!gb.get_flag(Flags::N));
    assert!(!gb.get_flag(Flags::C));
}
