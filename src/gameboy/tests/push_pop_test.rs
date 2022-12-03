use crate::gameboy::opcodes::Argument;
use crate::gameboy::opcodes::Category;
use crate::gameboy::tests::decode_util::decode;
use crate::gameboy::Gameboy;
use crate::gameboy::OpCode;
use crate::gameboy::RegisterLabel16;

#[test]
fn pop_instruction_moves_the_stack_pointer() {
    // POP BC. The test has space then the
    let mut gb = Gameboy::new(vec![0xC1, 0x00, 0x01, 0x23]);

    // setup the stack pointer
    gb.set_register_16(RegisterLabel16::StackPointer, 0x02);

    let cycles = gb.step_once();

    // section("The stack shrinks upwards") {
    // The stack grows downwards so the pop instruction
    // moves the stack pointer upwards
    assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0x4);

    // section("The instruction takes 12 cycles and 1 byte") {
    assert_eq!(cycles, 12);
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1);

    // section("The contents of the stack are put in BC") {
    let bc = gb.get_register_16(RegisterLabel16::BC);
    assert_eq!(bc, 0x2301);
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
fn push_instruction_decoding() {
    assert_eq!(
        decode(&[0xC5]),
        OpCode::new(
            Category::PUSH,
            [
                Argument::Register16Constant(RegisterLabel16::BC),
                Argument::None
            ]
        )
    );
    assert_eq!(
        decode(&[0xD5]),
        OpCode::new(
            Category::PUSH,
            [
                Argument::Register16Constant(RegisterLabel16::DE),
                Argument::None
            ]
        )
    );
    assert_eq!(
        decode(&[0xE5]),
        OpCode::new(
            Category::PUSH,
            [
                Argument::Register16Constant(RegisterLabel16::HL),
                Argument::None
            ]
        )
    );
    assert_eq!(
        decode(&[0xF5]),
        OpCode::new(
            Category::PUSH,
            [
                Argument::Register16Constant(RegisterLabel16::AF),
                Argument::None
            ]
        )
    );
}

#[test]
fn pop_instruction_decoding() {
    let pop = |register| {
        OpCode::new(
            Category::POP,
            [Argument::Register16Constant(register), Argument::None],
        )
    };

    assert_eq!(decode(&[0xC1]), pop(RegisterLabel16::BC));
    assert_eq!(decode(&[0xD1]), pop(RegisterLabel16::DE));
    assert_eq!(decode(&[0xE1]), pop(RegisterLabel16::HL));
    assert_eq!(decode(&[0xF1]), pop(RegisterLabel16::AF));
}
