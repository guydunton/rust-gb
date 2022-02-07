use crate::gameboy::{
    cpu::CPU,
    memory_adapter::MemoryAdapter,
    opcodes::{Argument, Category},
    read_flag, Flags, Gameboy, OpCode, RegisterLabel16, RegisterLabel8,
};

use super::decode_util::decode;

#[test]
fn xor_instruction() {
    let mut gb = Gameboy::new(vec![0xAF]);

    gb.set_register_8(RegisterLabel8::A, 0x01);
    gb.set_register_8(RegisterLabel8::F, 0b1111_0000);

    let cycles = gb.step_once();

    assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x00);
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

    assert_eq!(gb.get_flag(Flags::Z), true);
    assert_eq!(gb.get_flag(Flags::C), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(cycles, 4);
}

fn new_xor(register: RegisterLabel8) -> OpCode {
    OpCode::new(
        Category::XOR,
        [Argument::Register8Constant(register), Argument::None],
    )
}

#[test]
fn xor_sets_flags_based_on_result() {
    let opcode = new_xor(RegisterLabel8::A);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0x01);
    cpu.write_8_bits(RegisterLabel8::F, 0b1111_0000);

    let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x00);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);

    assert_eq!(read_flag(&cpu, Flags::Z), true);
    assert_eq!(read_flag(&cpu, Flags::C), false);
    assert_eq!(read_flag(&cpu, Flags::H), false);
    assert_eq!(read_flag(&cpu, Flags::N), false);
    assert_eq!(cycles, 4);
}

#[test]
fn xor_sets_a_to_result() {
    let opcode = new_xor(RegisterLabel8::B);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0x01);
    cpu.write_8_bits(RegisterLabel8::B, 0x00);
    cpu.write_8_bits(RegisterLabel8::F, 0b1111_0000);

    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x01);

    assert_eq!(read_flag(&cpu, Flags::Z), false);
}

#[test]
fn decode_xor_test() {
    let xor_opcode = |register| {
        OpCode::new(
            Category::XOR,
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

    assert_eq!(decode(&[0xA8]), xor_opcode(B));
    assert_eq!(decode(&[0xA9]), xor_opcode(C));
    assert_eq!(decode(&[0xAA]), xor_opcode(D));
    assert_eq!(decode(&[0xAB]), xor_opcode(E));
    assert_eq!(decode(&[0xAC]), xor_opcode(H));
    assert_eq!(decode(&[0xAD]), xor_opcode(L));
    assert_eq!(decode(&[0xAF]), xor_opcode(A));
}
