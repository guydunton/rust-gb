use crate::gameboy::{
    cpu::CPU,Gameboy,
    memory_adapter::MemoryAdapter,
    opcodes::{Argument, Category},
    Flags, OpCode, RegisterLabel16, RegisterLabel8,
    read_flag
};

#[test]
fn run_cpl() {
    let opcode = OpCode::new(Category::CPL, [Argument::None, Argument::None]);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    let a_value = 0xAA;

    cpu.write_8_bits(RegisterLabel8::A, a_value);

    let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cycles, 4);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 1);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), a_value^a_value);

    assert_eq!(read_flag(&cpu, Flags::N), true);
    assert_eq!(read_flag(&cpu, Flags::H), true);
}
