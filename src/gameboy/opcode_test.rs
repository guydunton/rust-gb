use gameboy::opcode_library::{ decode_instruction, OpCode };
use gameboy::cpu::CPU;
use gameboy::register::{ RegisterLabel8, RegisterLabel16 };
use gameboy::read_write_register::ReadWriteRegister;

macro_rules! setup_cpu {
    ( [ $( $x:expr ),* ] , $cpu:ident , $memory:ident, $opcode:ident ) => {
        let mut $memory = vec![$($x,)*];
        let $opcode = decode_instruction(0, &$memory);
        let mut $cpu = CPU::new();
    }
}

macro_rules! run_cpu {
    ( $cpu:ident, $memory:ident, $opcode:ident ) => {
        $opcode.run::<CPU>(&mut $cpu, &mut $memory);
    }
}

#[test]
fn load16_instructions() {
    { // LD SP d16
        setup_cpu!([0x31, 0xFE, 0xFF], cpu, memory, opcode);

        run_cpu!(cpu, memory, opcode);

        assert_eq!(cpu.read_16_bits(RegisterLabel16::StackPointer), 0xFFFE);
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x0003);
    }
    { // LD HL d16
        setup_cpu!([0x21, 0xFF, 0x9F], cpu, memory, opcode);
        run_cpu!(cpu, memory, opcode);

        assert_eq!(cpu.read_16_bits(RegisterLabel16::HL), 0x9FFF);
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x0003);
    }
}

#[test]
fn load8_instructions() {
    setup_cpu!([0x32, 0x00], cpu, memory, opcode); // LD (HL-) A
    cpu.write_16_bits(RegisterLabel16::HL, 0x0001);
    cpu.write_8_bits(RegisterLabel8::A, 0x01);
    run_cpu!(cpu, memory, opcode);

    assert_eq!(cpu.read_16_bits(RegisterLabel16::HL), 0x0000);
    assert_eq!(memory[1], 0x01);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x0001);
}

#[test]
fn xor_instruction() {
    setup_cpu!([0xAF], cpu, memory, opcode);

    cpu.write_8_bits(RegisterLabel8::A, 0x01);
    cpu.write_8_bits(RegisterLabel8::F, 0b1111_0000);

    run_cpu!(cpu, memory, opcode);

    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x00);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);

    assert_eq!(cpu.read_8_bits(RegisterLabel8::F), 0x00);
}