use crate::gameboy::cpu::CPU;
use crate::gameboy::memory_adapter::MemoryAdapter;
use crate::gameboy::opcodes::{Argument, Category};
use crate::gameboy::{read_flag, write_flag, Flags, OpCode, RegisterLabel8};

use super::decode_util::decode;

#[test]
fn decode_adc_instruction() {
    let adc_r8 = |register| {
        OpCode::new(
            Category::ADC,
            [Argument::Register8Constant(register), Argument::None],
        )
    };
    assert_eq!(decode(&[0x88]), adc_r8(RegisterLabel8::B));
    assert_eq!(decode(&[0x89]), adc_r8(RegisterLabel8::C));
    assert_eq!(decode(&[0x8A]), adc_r8(RegisterLabel8::D));
    assert_eq!(decode(&[0x8B]), adc_r8(RegisterLabel8::E));
    assert_eq!(decode(&[0x8C]), adc_r8(RegisterLabel8::H));
    assert_eq!(decode(&[0x8D]), adc_r8(RegisterLabel8::L));
    assert_eq!(decode(&[0x8F]), adc_r8(RegisterLabel8::A));
}

#[test]
fn test_adc_instruction_size() {
    let opcode = OpCode::new(
        Category::ADC,
        [
            Argument::Register8Constant(RegisterLabel8::A),
            Argument::None,
        ],
    );
    assert_eq!(opcode.size(), 1);
}

#[test]
fn test_adc_sets_zero_flag_when_result_is_zero() {
    let opcode = OpCode::new(
        Category::ADC,
        [
            Argument::Register8Constant(RegisterLabel8::B),
            Argument::None,
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0; 0xFFFF];

    // Set A to 0xFE and B to 0x01, with carry flag set
    // This will result in 0xFE + 0x01 + 1 = 0x100 (0x00 with carry)
    cpu.write_8_bits(RegisterLabel8::A, 0xFE);
    cpu.write_8_bits(RegisterLabel8::B, 0x01);
    write_flag(&mut cpu, Flags::C, true);

    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x00);
    assert_eq!(read_flag(&cpu, Flags::Z), true);
    assert_eq!(read_flag(&cpu, Flags::C), true);
}

#[test]
fn test_adc_clears_zero_flag_when_result_is_not_zero() {
    let opcode = OpCode::new(
        Category::ADC,
        [
            Argument::Register8Constant(RegisterLabel8::B),
            Argument::None,
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0; 0xFFFF];

    // Set A to 0x01 and B to 0x02, with carry flag set
    // This will result in 0x01 + 0x02 + 1 = 0x04
    cpu.write_8_bits(RegisterLabel8::A, 0x01);
    cpu.write_8_bits(RegisterLabel8::B, 0x02);
    write_flag(&mut cpu, Flags::C, true);

    // Set Z flag initially to verify it gets cleared
    write_flag(&mut cpu, Flags::Z, true);

    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x04);
    assert_eq!(read_flag(&cpu, Flags::Z), false);
    assert_eq!(read_flag(&cpu, Flags::C), false);
}

#[test]
fn test_adc_clears_n_flag() {
    let opcode = OpCode::new(
        Category::ADC,
        [
            Argument::Register8Constant(RegisterLabel8::B),
            Argument::None,
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0; 0xFFFF];

    // Set N flag initially to verify it gets cleared
    write_flag(&mut cpu, Flags::N, true);

    cpu.write_8_bits(RegisterLabel8::A, 0x01);
    cpu.write_8_bits(RegisterLabel8::B, 0x02);

    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(read_flag(&cpu, Flags::N), false);
}

#[test]
fn test_adc_adds_carry_when_both_operands_are_ff() {
    let opcode = OpCode::new(
        Category::ADC,
        [
            Argument::Register8Constant(RegisterLabel8::B),
            Argument::None,
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::A, 0xFF);
    cpu.write_8_bits(RegisterLabel8::B, 0xFF);
    write_flag(&mut cpu, Flags::C, true);

    opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(read_flag(&cpu, Flags::C), true);
    assert_eq!(read_flag(&cpu, Flags::H), true);
    assert_eq!(read_flag(&cpu, Flags::Z), false);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0xFF);
}
