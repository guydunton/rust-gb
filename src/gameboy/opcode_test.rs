use gameboy::opcode_library::{ decode_instruction, OpCode };
use gameboy::cpu::CPU;
use gameboy::register::{ RegisterLabel8, RegisterLabel16 };
use gameboy::read_write_register::ReadWriteRegister;

#[test]
fn load_instruction() {
    let mut memory = vec![0x31, 0xFE, 0xFF];
    let opcode = decode_instruction(0, &memory);

    let mut cpu = CPU::new();
    opcode.run::<CPU>(&mut cpu, &mut memory);

    assert_eq!(cpu.read_16_bits(RegisterLabel16::StackPointer), 0xFFFE);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x0003);
}