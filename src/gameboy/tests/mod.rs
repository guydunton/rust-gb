mod add_test;
mod alu_test;
mod cp_test;
mod dec_test;
mod inc_test;
mod jump_test;
mod load16_test;
mod load8_test;
mod opcode_printer_test;
mod pop_test;
mod ppu_test;
mod ret_test;
mod sub_test;
mod timing;

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

#[test]
fn xor_instruction() {
    let mut gb = Gameboy::new(vec![0xAF]);

    gb.set_register_8(RegisterLabel8::A, 0x01);
    gb.set_register_8(RegisterLabel8::F, 0b1111_0000);

    let cycles = gb.step_once();

    assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x00);
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

    assert_eq!(gb.get_register_8(RegisterLabel8::F), 0x00);
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
        let cycles = gb.step_once();

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
        let cycles = gb.step_once();

        assert_eq!(gb.get_flag(Flags::Z), true);
        assert_eq!(cycles, 12);
    }
}

#[test]
fn nop_instruction() {
    let mut gb = Gameboy::new(vec![0x00]);
    let cycles = gb.step_once();

    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1);
    assert_eq!(cycles, 4);
}

#[test]
fn call_moves_the_program_counter_to_the_call_location() {
    // call 0x0004        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06
    let mut gb = Gameboy::new(vec![0xCD, 0x04, 0x00, 0x03, 0x04, 0x05, 0x06]);
    gb.set_register_16(RegisterLabel16::StackPointer, 0x07);

    /*
    0x00 : instruction
    0x01 : arg part 2
    0x02 : arg part 1
    0x03 : return point and location added to the stack
    0x04 : point where we call to
    0x05 : Part 2 of the stack
    0x06 : part 1 of the stack
    */
    let _ = gb.step_once();
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x04);
}

#[test]
fn call_sets_the_stack_value_correctly() {
    // call 0x0004        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06
    let mut gb = Gameboy::new(vec![0xCD, 0x04, 0x00, 0x03, 0x04, 0x05, 0x06]);
    gb.set_register_16(RegisterLabel16::StackPointer, 0x07);

    /*
    0x00 : instruction
    0x01 : arg part 2
    0x02 : arg part 1
    0x03 : return point and location added to the stack
    0x04 : point where we call to
    0x05 : Part 2 of the stack
    0x06 : part 1 of the stack
    */
    // We push 0x0003 onto the stack
    // decrements stack and pushed 0x00
    // decrements again and pushed 0x03
    let _ = gb.step_once();
    assert_eq!(gb.get_memory_at(0x05), 0x03);
    assert_eq!(gb.get_memory_at(0x06), 0x00);

    assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0x05);
}

#[test]
fn call_instruction_takes_24_cycles() {
    // call 0x0004        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06
    let mut gb = Gameboy::new(vec![0xCD, 0x04, 0x00, 0x03, 0x04, 0x05, 0x06]);
    gb.set_register_16(RegisterLabel16::StackPointer, 0x07);

    /*
    0x00 : instruction
    0x01 : arg part 2
    0x02 : arg part 1
    0x03 : return point and location added to the stack
    0x04 : point where we call to
    0x05 : Part 2 of the stack
    0x06 : part 1 of the stack
    */
    let cycles = gb.step_once();
    assert_eq!(cycles, 24);
}

#[test]
fn push_instruction_tests_push_moves_2_bytes_onto_the_stack() {
    let mut gb = Gameboy::new(vec![0xC5, 0x00, 0x00]);
    gb.set_register_16(RegisterLabel16::BC, 0x1234);
    gb.set_register_16(RegisterLabel16::StackPointer, 0x03);

    let cycles = gb.step_once();

    assert_eq!(gb.get_memory_at(1), 0x34);
    assert_eq!(gb.get_memory_at(2), 0x12);

    assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0x01);

    // The cycles and the size are correct
    assert_eq!(cycles, 16);
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
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

    let cycles = gb.step_once();

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

    let cycles = gb.step_once();
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
