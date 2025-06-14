mod adc_test;
mod add_test;
mod alu_test;
mod and_test;
mod call_test;
mod cb_test;
mod cp_test;
mod cpl_test;
mod dec_test;
mod decode_util;
mod inc_test;
mod interrupt_instruction_tests;
mod jump_test;
mod load16_test;
mod load8_test;
mod memory_test;
mod misc_instructions_test;
mod opcode_printer_test;
mod or;
mod ppu_test;
mod push_pop_test;
mod ret_test;
mod sub_test;
mod timing;
mod xor_test;

use crate::gameboy::Gameboy;

fn infinite_loop_gb() -> Gameboy<'static> {
    // Each loop will be 16 clocks & take 2 steps
    // NOP
    // JR -3
    let gb = Gameboy::new(vec![0x00, 0x18, 0xFD]);
    gb
}

use crate::gameboy::flags_register::*;
use crate::gameboy::register::{RegisterLabel16, RegisterLabel8};
use crate::gameboy::Labels;

#[test]
fn i_can_access_all_parts_of_memory() {
    let mut gb = Gameboy::new(vec![]);

    gb.set_memory_at(0x00, 1);
    gb.set_memory_at(0xFFFF, 2);

    assert_eq!(gb.get_memory_at(0x00), 1);
    assert_eq!(gb.get_memory_at(0xFFFF), 2);
}

#[test]
fn xor_instruction() {
    let mut gb = Gameboy::new(vec![0xAF]);

    gb.set_register_8(RegisterLabel8::A, 0x01);
    gb.set_register_8(RegisterLabel8::F, 0b1111_0000);

    let cycles = gb.step_once().unwrap();

    assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x00);
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

    assert_eq!(gb.get_flag(Flags::Z), true);
    assert_eq!(gb.get_flag(Flags::C), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(cycles, 4);
}

#[test]
fn bit_instruction() {
    // BIT 7,H
    {
        // Check the bit flag when the bit is already set to 1
        let mut gb = Gameboy::new(vec![0xCB, 0x7C]);
        gb.set_register_8(RegisterLabel8::H, 0b1000_0000);
        let carry_flag = gb.get_flag(Flags::C);
        let cycles = gb.step_once().unwrap();

        assert_eq!(gb.get_flag(Flags::Z), false);
        assert_eq!(gb.get_flag(Flags::N), false);
        assert_eq!(gb.get_flag(Flags::H), true);
        assert_eq!(gb.get_flag(Flags::C), carry_flag); // The carry flag is unaffected

        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x2);
        assert_eq!(cycles, 12);
    }
    {
        // Check the bit flag when the bit is 0
        let mut gb = Gameboy::new(vec![0xCB, 0x7C]);
        gb.set_register_8(RegisterLabel8::H, 0x0);
        let cycles = gb.step_once().unwrap();

        assert_eq!(gb.get_flag(Flags::Z), true);
        assert_eq!(cycles, 12);
    }
}

#[test]
fn nop_instruction() {
    let mut gb = Gameboy::new(vec![0x00]);
    let cycles = gb.step_once().unwrap();

    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1);
    assert_eq!(cycles, 4);
}

#[test]
fn rotate_left_shifts_along_with_the_carry_flag() {
    let mut gb = Gameboy::new(vec![0xCB, 0x11]);
    // C  C register
    // 1  0101_0101
    // After
    // 0  1010_1011
    gb.set_flag(Flags::C, true);
    gb.set_register_8(RegisterLabel8::C, 0b0101_0101);

    let cycles = gb.step_once().unwrap();

    assert_eq!(gb.get_register_8(RegisterLabel8::C), 0b1010_1011);
    assert_eq!(gb.get_flag(Flags::C), false);

    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::Z), false);

    assert_eq!(cycles, 8);
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x2);

    // Run again to test the carry flag
    // Before:
    // 0  1010_1011
    // 1  0101_0110
    gb.set_register_16(RegisterLabel16::ProgramCounter, 0x0);

    gb.set_flag(Flags::H, true);
    gb.set_flag(Flags::N, true);

    let _ = gb.step_once();
    assert_eq!(gb.get_flag(Flags::C), true);

    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::N), false);
}

#[test]
fn rotate_left_sets_the_zero_flag_if_the_result_is_0() {
    let mut gb = Gameboy::new(vec![0xCB, 0x11]);

    let _ = gb.step_once();
    assert_eq!(gb.get_flag(Flags::Z), true);
}

#[test]
fn rla_cycles_the_a_register_left_through_carry() {
    let mut gb = Gameboy::new(vec![0x17]);

    // Before run:
    // C A
    // 1 0101_0101
    // After run:
    // 0 1010_1011
    gb.set_register_8(RegisterLabel8::A, 0b0101_0101);
    gb.set_flag(Flags::C, true);

    let cycles = gb.step_once().unwrap();
    assert_eq!(cycles, 4);
    assert_eq!(gb.get_register_8(RegisterLabel8::A), 0b1010_1011);
    assert_eq!(gb.get_flag(Flags::C), false);

    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
}

#[test]
fn sets_all_flags_to_false_even_if_they_are_set() {
    let mut gb = Gameboy::new(vec![0x17]);

    // Before run:
    // C A
    // 1 0101_0101
    // After run:
    // 0 1010_1011
    gb.set_register_8(RegisterLabel8::A, 0b0101_0101);
    gb.set_flag(Flags::C, true);

    gb.set_flag(Flags::Z, true);
    gb.set_flag(Flags::N, true);
    gb.set_flag(Flags::H, true);

    let _ = gb.step_once();
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
}

#[test]
fn get_memory_slice_at_works() {
    let gb = Gameboy::new(vec![0x01, 0x02]);

    assert_eq!(gb.get_memory_slice_at(0x00, 0x02), [0x01, 0x02]);
}

#[test]
fn set_ff50_to_disable_bootloader() {
    let audio = |_| {};
    let mut gb = Gameboy::new_with_bootloader(audio, &vec![0x01; 32_000]);

    assert_ne!(gb.get_memory_at(0x00), 0x01);
    gb.set_memory_at(Labels::BOOTLOADER_DISABLE, 1);
    assert_eq!(gb.get_memory_at(0x00), 0x01);
}

#[test]
fn run_ldh50_to_disable_bootloader() {
    let audio = |_| {};
    let mut gb = Gameboy::new_with_bootloader(audio, &vec![0x01; 32_000]);
    gb.set_memory_at(0x0, 0xE0);
    gb.set_memory_at(0x01, 0x50);

    gb.step_once();

    assert_eq!(gb.get_memory_at(0x00), 0x01);
}
