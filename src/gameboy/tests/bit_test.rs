use crate::gameboy::{
    Flags, OpCode, RegisterLabel8, RegisterLabel16,
    cpu::CPU,
    memory_adapter::MemoryAdapter,
    opcodes::{Argument, Category},
    read_flag,
    tests::decode_util::decode,
};

const A: Argument = Argument::Register8Constant(RegisterLabel8::A);
const B: Argument = Argument::Register8Constant(RegisterLabel8::B);
const C: Argument = Argument::Register8Constant(RegisterLabel8::C);
const D: Argument = Argument::Register8Constant(RegisterLabel8::D);
const E: Argument = Argument::Register8Constant(RegisterLabel8::E);
const H: Argument = Argument::Register8Constant(RegisterLabel8::H);
const L: Argument = Argument::Register8Constant(RegisterLabel8::L);
const HL: Argument = Argument::RegisterIndirect(RegisterLabel16::HL);

fn bit(bit_value: u8, target: Argument) -> OpCode {
    OpCode::new(Category::BIT, [Argument::Bit(bit_value), target])
}

#[test]
fn bit_n_r8_works_correctly_when_bit_is_set() {
    let opcode = bit(3, B);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::B, 0b0000_1000);

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x02);

    assert_eq!(read_flag(&cpu, Flags::Z), false);
    assert_eq!(read_flag(&cpu, Flags::C), false);
    assert_eq!(read_flag(&cpu, Flags::H), true);
    assert_eq!(read_flag(&cpu, Flags::N), false);
    assert_eq!(cycles, 8);
}

#[test]
fn bit_n_r8_works_correctly_when_bit_is_unset() {
    let opcode = bit(3, B);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::B, 0b0000_0000);

    opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(read_flag(&cpu, Flags::Z), true);
    assert_eq!(read_flag(&cpu, Flags::C), false);
    assert_eq!(read_flag(&cpu, Flags::H), true);
    assert_eq!(read_flag(&cpu, Flags::N), false);
}

#[test]
fn bit_n_hl_works_correctly_when_bit_is_set() {
    let opcode = bit(5, HL);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_16_bits(RegisterLabel16::HL, 0x00C0);
    memory[0x00C0] = 0b0010_0000;

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x02);

    assert_eq!(read_flag(&cpu, Flags::Z), false);
    assert_eq!(read_flag(&cpu, Flags::C), false);
    assert_eq!(read_flag(&cpu, Flags::H), true);
    assert_eq!(read_flag(&cpu, Flags::N), false);
    assert_eq!(cycles, 12);
}

#[test]
fn bit_n_hl_works_correctly_when_bit_is_unset() {
    let opcode = bit(5, HL);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_16_bits(RegisterLabel16::HL, 0x00C0);
    memory[0x00C0] = 0b0000_0000;

    opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(read_flag(&cpu, Flags::Z), true);
    assert_eq!(read_flag(&cpu, Flags::C), false);
    assert_eq!(read_flag(&cpu, Flags::H), true);
    assert_eq!(read_flag(&cpu, Flags::N), false);
}

#[test]
fn decode_bit_instructions() {
    assert_eq!(decode(&[0xCB, 0x40]), bit(0, B));
    assert_eq!(decode(&[0xCB, 0x41]), bit(0, C));
    assert_eq!(decode(&[0xCB, 0x42]), bit(0, D));
    assert_eq!(decode(&[0xCB, 0x43]), bit(0, E));
    assert_eq!(decode(&[0xCB, 0x44]), bit(0, H));
    assert_eq!(decode(&[0xCB, 0x45]), bit(0, L));
    assert_eq!(decode(&[0xCB, 0x46]), bit(0, HL));
    assert_eq!(decode(&[0xCB, 0x47]), bit(0, A));
    assert_eq!(decode(&[0xCB, 0x48]), bit(1, B));
    assert_eq!(decode(&[0xCB, 0x49]), bit(1, C));
    assert_eq!(decode(&[0xCB, 0x4A]), bit(1, D));
    assert_eq!(decode(&[0xCB, 0x4B]), bit(1, E));
    assert_eq!(decode(&[0xCB, 0x4C]), bit(1, H));
    assert_eq!(decode(&[0xCB, 0x4D]), bit(1, L));
    assert_eq!(decode(&[0xCB, 0x4E]), bit(1, HL));
    assert_eq!(decode(&[0xCB, 0x4F]), bit(1, A));
    assert_eq!(decode(&[0xCB, 0x50]), bit(2, B));
    assert_eq!(decode(&[0xCB, 0x51]), bit(2, C));
    assert_eq!(decode(&[0xCB, 0x52]), bit(2, D));
    assert_eq!(decode(&[0xCB, 0x53]), bit(2, E));
    assert_eq!(decode(&[0xCB, 0x54]), bit(2, H));
    assert_eq!(decode(&[0xCB, 0x55]), bit(2, L));
    assert_eq!(decode(&[0xCB, 0x56]), bit(2, HL));
    assert_eq!(decode(&[0xCB, 0x57]), bit(2, A));
    assert_eq!(decode(&[0xCB, 0x58]), bit(3, B));
    assert_eq!(decode(&[0xCB, 0x59]), bit(3, C));
    assert_eq!(decode(&[0xCB, 0x5A]), bit(3, D));
    assert_eq!(decode(&[0xCB, 0x5B]), bit(3, E));
    assert_eq!(decode(&[0xCB, 0x5C]), bit(3, H));
    assert_eq!(decode(&[0xCB, 0x5D]), bit(3, L));
    assert_eq!(decode(&[0xCB, 0x5E]), bit(3, HL));
    assert_eq!(decode(&[0xCB, 0x5F]), bit(3, A));
    assert_eq!(decode(&[0xCB, 0x60]), bit(4, B));
    assert_eq!(decode(&[0xCB, 0x61]), bit(4, C));
    assert_eq!(decode(&[0xCB, 0x62]), bit(4, D));
    assert_eq!(decode(&[0xCB, 0x63]), bit(4, E));
    assert_eq!(decode(&[0xCB, 0x64]), bit(4, H));
    assert_eq!(decode(&[0xCB, 0x65]), bit(4, L));
    assert_eq!(decode(&[0xCB, 0x66]), bit(4, HL));
    assert_eq!(decode(&[0xCB, 0x67]), bit(4, A));
    assert_eq!(decode(&[0xCB, 0x68]), bit(5, B));
    assert_eq!(decode(&[0xCB, 0x69]), bit(5, C));
    assert_eq!(decode(&[0xCB, 0x6A]), bit(5, D));
    assert_eq!(decode(&[0xCB, 0x6B]), bit(5, E));
    assert_eq!(decode(&[0xCB, 0x6C]), bit(5, H));
    assert_eq!(decode(&[0xCB, 0x6D]), bit(5, L));
    assert_eq!(decode(&[0xCB, 0x6E]), bit(5, HL));
    assert_eq!(decode(&[0xCB, 0x6F]), bit(5, A));
    assert_eq!(decode(&[0xCB, 0x70]), bit(6, B));
    assert_eq!(decode(&[0xCB, 0x71]), bit(6, C));
    assert_eq!(decode(&[0xCB, 0x72]), bit(6, D));
    assert_eq!(decode(&[0xCB, 0x73]), bit(6, E));
    assert_eq!(decode(&[0xCB, 0x74]), bit(6, H));
    assert_eq!(decode(&[0xCB, 0x75]), bit(6, L));
    assert_eq!(decode(&[0xCB, 0x76]), bit(6, HL));
    assert_eq!(decode(&[0xCB, 0x77]), bit(6, A));
    assert_eq!(decode(&[0xCB, 0x78]), bit(7, B));
    assert_eq!(decode(&[0xCB, 0x79]), bit(7, C));
    assert_eq!(decode(&[0xCB, 0x7A]), bit(7, D));
    assert_eq!(decode(&[0xCB, 0x7B]), bit(7, E));
    assert_eq!(decode(&[0xCB, 0x7C]), bit(7, H));
    assert_eq!(decode(&[0xCB, 0x7D]), bit(7, L));
    assert_eq!(decode(&[0xCB, 0x7E]), bit(7, HL));
    assert_eq!(decode(&[0xCB, 0x7F]), bit(7, A));
}
