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

    let and_op_r8 = |register| {
        OpCode::new(
            Category::AND,
            [Argument::Register8Constant(register), Argument::None],
        )
    };

    const A: RegisterLabel8 = RegisterLabel8::A;
    const B: RegisterLabel8 = RegisterLabel8::B;
    const C: RegisterLabel8 = RegisterLabel8::C;
    const D: RegisterLabel8 = RegisterLabel8::D;
    const E: RegisterLabel8 = RegisterLabel8::E;
    const H: RegisterLabel8 = RegisterLabel8::H;
    const L: RegisterLabel8 = RegisterLabel8::L;

    assert_eq!(decode(&[0xA0]), and_op_r8(B));
    assert_eq!(decode(&[0xA1]), and_op_r8(C));
    assert_eq!(decode(&[0xA2]), and_op_r8(D));
    assert_eq!(decode(&[0xA3]), and_op_r8(E));
    assert_eq!(decode(&[0xA4]), and_op_r8(H));
    assert_eq!(decode(&[0xA5]), and_op_r8(L));
    assert_eq!(decode(&[0xA7]), and_op_r8(A));

    assert_eq!(
        decode(&[0xA6]),
        OpCode::new(
            Category::AND,
            [
                Argument::RegisterIndirect(RegisterLabel16::HL),
                Argument::None
            ]
        )
    );
}

#[test]
fn and_instruction_works_with_registers() {
    let opcode = OpCode::new(
        Category::AND,
        [
            Argument::Register8Constant(RegisterLabel8::B),
            Argument::None,
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0b0000_0011);
    cpu.write_8_bits(RegisterLabel8::B, 0b0000_0001);

    let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0b_0000_0001);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);
    assert_eq!(cycles, 4);
}

#[test]
fn and_instruction_works_with_indirection() {
    let opcode = OpCode::new(
        Category::AND,
        [
            Argument::RegisterIndirect(RegisterLabel16::HL),
            Argument::None,
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0b0000_0011);
    cpu.write_16_bits(RegisterLabel16::HL, 0xFF01);
    memory[0xFF01] = 0b0000_0001;

    let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0b_0000_0001);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);
    assert_eq!(cycles, 8);
}
