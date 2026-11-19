use crate::gameboy::{
    OpCode, RegisterLabel8, RegisterLabel16,
    cpu::CPU,
    memory_adapter::MemoryAdapter,
    opcodes::{Argument, Category},
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

fn set(bit_value: u8, target: Argument) -> OpCode {
    OpCode::new(Category::SET, [Argument::Bit(bit_value), target])
}

#[test]
fn set_n_r8_works_correctly() {
    let opcode = set(0, B);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x02);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::B), 1);
    assert_eq!(cycles, 8);
}

#[test]
fn set_n_hl_works_correctly() {
    let opcode = set(0, HL);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_16_bits(RegisterLabel16::HL, 0xC0);

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x02);
    assert_eq!(memory[0xC0], 1);
    assert_eq!(cycles, 16);
}

#[test]
fn decode_set_test() {
    assert_eq!(decode(&[0xCB, 0xC0]), set(0, B));
    assert_eq!(decode(&[0xCB, 0xC1]), set(0, C));
    assert_eq!(decode(&[0xCB, 0xC2]), set(0, D));
    assert_eq!(decode(&[0xCB, 0xC3]), set(0, E));
    assert_eq!(decode(&[0xCB, 0xC4]), set(0, H));
    assert_eq!(decode(&[0xCB, 0xC5]), set(0, L));
    assert_eq!(decode(&[0xCB, 0xC6]), set(0, HL));
    assert_eq!(decode(&[0xCB, 0xC7]), set(0, A));
    assert_eq!(decode(&[0xCB, 0xC8]), set(1, B));
    assert_eq!(decode(&[0xCB, 0xC9]), set(1, C));
    assert_eq!(decode(&[0xCB, 0xCA]), set(1, D));
    assert_eq!(decode(&[0xCB, 0xCB]), set(1, E));
    assert_eq!(decode(&[0xCB, 0xCC]), set(1, H));
    assert_eq!(decode(&[0xCB, 0xCD]), set(1, L));
    assert_eq!(decode(&[0xCB, 0xCE]), set(1, HL));
    assert_eq!(decode(&[0xCB, 0xCF]), set(1, A));
    assert_eq!(decode(&[0xCB, 0xD0]), set(2, B));
    assert_eq!(decode(&[0xCB, 0xD1]), set(2, C));
    assert_eq!(decode(&[0xCB, 0xD2]), set(2, D));
    assert_eq!(decode(&[0xCB, 0xD3]), set(2, E));
    assert_eq!(decode(&[0xCB, 0xD4]), set(2, H));
    assert_eq!(decode(&[0xCB, 0xD5]), set(2, L));
    assert_eq!(decode(&[0xCB, 0xD6]), set(2, HL));
    assert_eq!(decode(&[0xCB, 0xD7]), set(2, A));
    assert_eq!(decode(&[0xCB, 0xD8]), set(3, B));
    assert_eq!(decode(&[0xCB, 0xD9]), set(3, C));
    assert_eq!(decode(&[0xCB, 0xDA]), set(3, D));
    assert_eq!(decode(&[0xCB, 0xDB]), set(3, E));
    assert_eq!(decode(&[0xCB, 0xDC]), set(3, H));
    assert_eq!(decode(&[0xCB, 0xDD]), set(3, L));
    assert_eq!(decode(&[0xCB, 0xDE]), set(3, HL));
    assert_eq!(decode(&[0xCB, 0xDF]), set(3, A));
    assert_eq!(decode(&[0xCB, 0xE0]), set(4, B));
    assert_eq!(decode(&[0xCB, 0xE1]), set(4, C));
    assert_eq!(decode(&[0xCB, 0xE2]), set(4, D));
    assert_eq!(decode(&[0xCB, 0xE3]), set(4, E));
    assert_eq!(decode(&[0xCB, 0xE4]), set(4, H));
    assert_eq!(decode(&[0xCB, 0xE5]), set(4, L));
    assert_eq!(decode(&[0xCB, 0xE6]), set(4, HL));
    assert_eq!(decode(&[0xCB, 0xE7]), set(4, A));
    assert_eq!(decode(&[0xCB, 0xE8]), set(5, B));
    assert_eq!(decode(&[0xCB, 0xE9]), set(5, C));
    assert_eq!(decode(&[0xCB, 0xEA]), set(5, D));
    assert_eq!(decode(&[0xCB, 0xEB]), set(5, E));
    assert_eq!(decode(&[0xCB, 0xEC]), set(5, H));
    assert_eq!(decode(&[0xCB, 0xED]), set(5, L));
    assert_eq!(decode(&[0xCB, 0xEE]), set(5, HL));
    assert_eq!(decode(&[0xCB, 0xEF]), set(5, A));
    assert_eq!(decode(&[0xCB, 0xF0]), set(6, B));
    assert_eq!(decode(&[0xCB, 0xF1]), set(6, C));
    assert_eq!(decode(&[0xCB, 0xF2]), set(6, D));
    assert_eq!(decode(&[0xCB, 0xF3]), set(6, E));
    assert_eq!(decode(&[0xCB, 0xF4]), set(6, H));
    assert_eq!(decode(&[0xCB, 0xF5]), set(6, L));
    assert_eq!(decode(&[0xCB, 0xF6]), set(6, HL));
    assert_eq!(decode(&[0xCB, 0xF7]), set(6, A));
    assert_eq!(decode(&[0xCB, 0xF8]), set(7, B));
    assert_eq!(decode(&[0xCB, 0xF9]), set(7, C));
    assert_eq!(decode(&[0xCB, 0xFA]), set(7, D));
    assert_eq!(decode(&[0xCB, 0xFB]), set(7, E));
    assert_eq!(decode(&[0xCB, 0xFC]), set(7, H));
    assert_eq!(decode(&[0xCB, 0xFD]), set(7, L));
    assert_eq!(decode(&[0xCB, 0xFE]), set(7, HL));
    assert_eq!(decode(&[0xCB, 0xFF]), set(7, A));
}
