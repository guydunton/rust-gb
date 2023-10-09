use crate::gameboy::{
    cpu::CPU,
    memory_adapter::MemoryAdapter,
    opcodes::{Argument, Category},
    read_flag, Flags, OpCode, RegisterLabel16, RegisterLabel8,
};

#[test]
fn run_cpl() {
    let opcode = OpCode::new(Category::CPL, [Argument::None, Argument::None]);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    let a_value = 0b1111_0000;

    cpu.write_8_bits(RegisterLabel8::A, a_value);

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cycles, 4);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 1);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0b0000_1111);

    assert_eq!(read_flag(&cpu, Flags::N), true);
    assert_eq!(read_flag(&cpu, Flags::H), true);
}
