use crate::gameboy::cpu::CPU;
use crate::gameboy::memory_adapter::MemoryAdapter;
use crate::gameboy::opcodes::Argument;
use crate::gameboy::opcodes::Category;
use crate::gameboy::opcodes::Decoder;
use crate::gameboy::read_flag;
use crate::gameboy::write_flag;
use crate::gameboy::Flags;
use crate::gameboy::OpCode;
use crate::gameboy::RegisterLabel16;
use crate::gameboy::RegisterLabel8;

fn inc_instruction(reg: RegisterLabel8) -> OpCode {
    OpCode::new(
        Category::INC,
        [Argument::Register8Constant(reg), Argument::None],
    )
}

#[test]
fn inc_increases_value_in_registry() {
    let opcode = inc_instruction(RegisterLabel8::B);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cycles, 4);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::B), 0x01);
}

#[test]
fn increment_can_cause_a_half_overflow() {
    let opcode = inc_instruction(RegisterLabel8::B);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::B, 0b1111);
    let _ = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(read_flag(&cpu, Flags::H), true);
}

#[test]
fn increment_from_max_causes_overflow() {
    let opcode = inc_instruction(RegisterLabel8::B);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_8_bits(RegisterLabel8::B, 0xFF);
    let _ = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(cpu.read_8_bits(RegisterLabel8::B), 0x0);
    assert_eq!(read_flag(&cpu, Flags::Z), true);
}

#[test]
fn increment_does_not_reset_flags_set_flags() {
    // Increment doesn't reset the Z and H if they are already set
    let opcode = inc_instruction(RegisterLabel8::C);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    write_flag(&mut cpu, Flags::Z, true);
    write_flag(&mut cpu, Flags::H, true);
    cpu.write_8_bits(RegisterLabel8::C, 0x1);

    let _ = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(read_flag(&cpu, Flags::Z), true);
    assert_eq!(read_flag(&cpu, Flags::H), true);
}

#[test]
fn n_flag_is_set_to_0() {
    let opcode = inc_instruction(RegisterLabel8::C);

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    write_flag(&mut cpu, Flags::N, true);
    let _ = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

    assert_eq!(read_flag(&cpu, Flags::N), false);
}

#[test]
fn inc_supports_hl_indirect() {
    let opcode = OpCode::new(
        Category::INC,
        [
            Argument::RegisterIndirect(RegisterLabel16::HL),
            Argument::None,
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x0; 0xFFFF];

    cpu.write_16_bits(RegisterLabel16::HL, 0xFF00);

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cycles, 12);
    assert_eq!(memory[0xFF00], 0x01);
}

#[test]
fn decode_all_increment_instructions() {
    let decode = |code| Decoder::decode_instruction(0x0, &[code]).unwrap();

    assert_eq!(inc_instruction(RegisterLabel8::B), decode(0x04));
    assert_eq!(inc_instruction(RegisterLabel8::D), decode(0x14));
    assert_eq!(inc_instruction(RegisterLabel8::H), decode(0x24));
    assert_eq!(inc_instruction(RegisterLabel8::C), decode(0x0C));
    assert_eq!(inc_instruction(RegisterLabel8::E), decode(0x1C));
    assert_eq!(inc_instruction(RegisterLabel8::L), decode(0x2C));
    assert_eq!(inc_instruction(RegisterLabel8::A), decode(0x3C));

    assert_eq!(
        OpCode::new(
            Category::INC,
            [
                Argument::RegisterIndirect(RegisterLabel16::HL),
                Argument::None
            ]
        ),
        decode(0x34)
    );
}

#[test]
fn inc_16_instruction() {
    use crate::gameboy::Gameboy;
    use crate::gameboy::{Flags, RegisterLabel16};

    // INC HL
    // INC DE
    let instructions: Vec<(u8, RegisterLabel16)> = vec![
        (0x03, RegisterLabel16::BC),
        (0x13, RegisterLabel16::DE),
        (0x23, RegisterLabel16::HL),
        (0x33, RegisterLabel16::StackPointer),
    ];

    for &(instruction, register) in instructions.iter() {
        let mut gb = Gameboy::new(vec![instruction]);
        let cycles = gb.step_once().unwrap();

        // Set the flags
        gb.set_flag(Flags::N, false);
        gb.set_flag(Flags::H, true);
        gb.set_flag(Flags::Z, false);
        gb.set_flag(Flags::C, true);

        // The 16 bit register should be changed
        assert_eq!(gb.get_register_16(register), 1);

        assert_eq!(cycles, 8);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

        // The flags should be unchanged
        assert_eq!(gb.get_flag(Flags::N), false);
        assert_eq!(gb.get_flag(Flags::H), true);
        assert_eq!(gb.get_flag(Flags::Z), false);
        assert_eq!(gb.get_flag(Flags::C), true);
    }
}
