use crate::gameboy::{
    cpu::CPU,
    memory_adapter::MemoryAdapter,
    opcodes::{Argument, Category, JumpCondition},
    write_flag, Gameboy, OpCode, RegisterLabel16,
};

use super::{decode_util::decode, Flags};

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
    let cycles = gb.step_once().unwrap();
    assert_eq!(cycles, 24);
}

#[test]
fn decode_call_instructions() {
    assert_eq!(
        decode(&[0xC4, 0x34, 0x12]),
        OpCode::new(
            Category::CALL,
            [
                Argument::JumpCondition(JumpCondition::NotZero),
                Argument::Label(0x1234),
            ],
        ),
    );
    assert_eq!(
        decode(&[0xD4, 0x34, 0x12]),
        OpCode::new(
            Category::CALL,
            [
                Argument::JumpCondition(JumpCondition::NotCarry),
                Argument::Label(0x1234),
            ],
        ),
    );
    assert_eq!(
        decode(&[0xCC, 0x34, 0x12]),
        OpCode::new(
            Category::CALL,
            [
                Argument::JumpCondition(JumpCondition::Zero),
                Argument::Label(0x1234),
            ],
        ),
    );
    assert_eq!(
        decode(&[0xDC, 0x34, 0x12]),
        OpCode::new(
            Category::CALL,
            [
                Argument::JumpCondition(JumpCondition::Carry),
                Argument::Label(0x1234),
            ],
        ),
    );
}

#[test]
fn call_jumps_if_condition_is_set() {
    let opcode = OpCode::new(
        Category::CALL,
        [
            Argument::JumpCondition(JumpCondition::Carry),
            Argument::Label(0x1234),
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_16_bits(RegisterLabel16::StackPointer, 0x1000);
    write_flag(&mut cpu, Flags::C, true);

    let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cycles.unwrap(), 24);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x1234);
}

#[test]
fn call_does_not_jump_if_condition_not_set() {
    let opcode = OpCode::new(
        Category::CALL,
        [
            Argument::JumpCondition(JumpCondition::Carry),
            Argument::Label(0x1234),
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_16_bits(RegisterLabel16::StackPointer, 0x1000);
    write_flag(&mut cpu, Flags::C, false);

    let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cycles.unwrap(), 12);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x0003);
}
