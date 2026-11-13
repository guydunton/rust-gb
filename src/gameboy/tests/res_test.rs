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

fn res(bit_value: u8, target: Argument) -> OpCode {
    OpCode::new(Category::RES, [Argument::Bit(bit_value), target])
}

#[test]
fn res_n_r8_works_correctly() {
    let opcode = res(0, B);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::B, 0b0000_0001);

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x02);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::B), 0);
    assert_eq!(cycles, 8);
}

#[test]
fn res_n_hl_works_correctly() {
    let opcode = res(0, HL);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_16_bits(RegisterLabel16::HL, 0xC0);
    memory[0xC0] = 0b0000_0001;

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x02);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::B), 0);
    assert_eq!(cycles, 16);
}

#[test]
fn decode_res_test() {
    assert_eq!(decode(&[0xCB, 0x80]), res(0, B));
    assert_eq!(decode(&[0xCB, 0x81]), res(0, C));
    assert_eq!(decode(&[0xCB, 0x82]), res(0, D));
    assert_eq!(decode(&[0xCB, 0x83]), res(0, E));
    assert_eq!(decode(&[0xCB, 0x84]), res(0, H));
    assert_eq!(decode(&[0xCB, 0x85]), res(0, L));
    assert_eq!(decode(&[0xCB, 0x86]), res(0, HL));
    assert_eq!(decode(&[0xCB, 0x87]), res(0, A));
    assert_eq!(decode(&[0xCB, 0x88]), res(1, B));
    assert_eq!(decode(&[0xCB, 0x89]), res(1, C));
    assert_eq!(decode(&[0xCB, 0x8A]), res(1, D));
    assert_eq!(decode(&[0xCB, 0x8B]), res(1, E));
    assert_eq!(decode(&[0xCB, 0x8C]), res(1, H));
    assert_eq!(decode(&[0xCB, 0x8D]), res(1, L));
    assert_eq!(decode(&[0xCB, 0x8E]), res(1, HL));
    assert_eq!(decode(&[0xCB, 0x8F]), res(1, A));
    assert_eq!(decode(&[0xCB, 0x90]), res(2, B));
    assert_eq!(decode(&[0xCB, 0x91]), res(2, C));
    assert_eq!(decode(&[0xCB, 0x92]), res(2, D));
    assert_eq!(decode(&[0xCB, 0x93]), res(2, E));
    assert_eq!(decode(&[0xCB, 0x94]), res(2, H));
    assert_eq!(decode(&[0xCB, 0x95]), res(2, L));
    assert_eq!(decode(&[0xCB, 0x96]), res(2, HL));
    assert_eq!(decode(&[0xCB, 0x97]), res(2, A));
    assert_eq!(decode(&[0xCB, 0x98]), res(3, B));
    assert_eq!(decode(&[0xCB, 0x99]), res(3, C));
    assert_eq!(decode(&[0xCB, 0x9A]), res(3, D));
    assert_eq!(decode(&[0xCB, 0x9B]), res(3, E));
    assert_eq!(decode(&[0xCB, 0x9C]), res(3, H));
    assert_eq!(decode(&[0xCB, 0x9D]), res(3, L));
    assert_eq!(decode(&[0xCB, 0x9E]), res(3, HL));
    assert_eq!(decode(&[0xCB, 0x9F]), res(3, A));
    assert_eq!(decode(&[0xCB, 0xA0]), res(4, B));
    assert_eq!(decode(&[0xCB, 0xA1]), res(4, C));
    assert_eq!(decode(&[0xCB, 0xA2]), res(4, D));
    assert_eq!(decode(&[0xCB, 0xA3]), res(4, E));
    assert_eq!(decode(&[0xCB, 0xA4]), res(4, H));
    assert_eq!(decode(&[0xCB, 0xA5]), res(4, L));
    assert_eq!(decode(&[0xCB, 0xA6]), res(4, HL));
    assert_eq!(decode(&[0xCB, 0xA7]), res(4, A));
    assert_eq!(decode(&[0xCB, 0xA8]), res(5, B));
    assert_eq!(decode(&[0xCB, 0xA9]), res(5, C));
    assert_eq!(decode(&[0xCB, 0xAA]), res(5, D));
    assert_eq!(decode(&[0xCB, 0xAB]), res(5, E));
    assert_eq!(decode(&[0xCB, 0xAC]), res(5, H));
    assert_eq!(decode(&[0xCB, 0xAD]), res(5, L));
    assert_eq!(decode(&[0xCB, 0xAE]), res(5, HL));
    assert_eq!(decode(&[0xCB, 0xAF]), res(5, A));
    assert_eq!(decode(&[0xCB, 0xB0]), res(6, B));
    assert_eq!(decode(&[0xCB, 0xB1]), res(6, C));
    assert_eq!(decode(&[0xCB, 0xB2]), res(6, D));
    assert_eq!(decode(&[0xCB, 0xB3]), res(6, E));
    assert_eq!(decode(&[0xCB, 0xB4]), res(6, H));
    assert_eq!(decode(&[0xCB, 0xB5]), res(6, L));
    assert_eq!(decode(&[0xCB, 0xB6]), res(6, HL));
    assert_eq!(decode(&[0xCB, 0xB7]), res(6, A));
    assert_eq!(decode(&[0xCB, 0xB8]), res(7, B));
    assert_eq!(decode(&[0xCB, 0xB9]), res(7, C));
    assert_eq!(decode(&[0xCB, 0xBA]), res(7, D));
    assert_eq!(decode(&[0xCB, 0xBB]), res(7, E));
    assert_eq!(decode(&[0xCB, 0xBC]), res(7, H));
    assert_eq!(decode(&[0xCB, 0xBD]), res(7, L));
    assert_eq!(decode(&[0xCB, 0xBE]), res(7, HL));
    assert_eq!(decode(&[0xCB, 0xBF]), res(7, A));
}
