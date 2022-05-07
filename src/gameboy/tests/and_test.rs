use crate::gameboy::{
    cpu::CPU,
    memory_adapter::MemoryAdapter,
    opcodes::{Argument, Category},
    read_flag, Flags, OpCode, RegisterLabel16, RegisterLabel8,
};

use super::decode_util::decode;

#[test]
fn and_with_d8_works_correctly() {
    let opcode = OpCode::new(
        Category::AND,
        [Argument::SmallValue(0b0000_0001), Argument::None],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0);

    let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x00);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x02);

    assert_eq!(read_flag(&cpu, Flags::Z), true);
    assert_eq!(read_flag(&cpu, Flags::C), false);
    assert_eq!(read_flag(&cpu, Flags::H), true);
    assert_eq!(read_flag(&cpu, Flags::N), false);
    assert_eq!(cycles, 8);
}

#[test]
fn and_with_d8_works_if_result_is_1() {
    let opcode = OpCode::new(
        Category::AND,
        [Argument::SmallValue(0b0000_0011), Argument::None],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0b0000_0001);

    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0b_0000_0001);

    assert_eq!(read_flag(&cpu, Flags::Z), false);
}

#[test]
fn decode_and_instruction() {
    assert_eq!(
        decode(&[0xE6, 0x01]),
        OpCode::new(Category::AND, [Argument::SmallValue(0x01), Argument::None])
    );
}
