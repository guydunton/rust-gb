use crate::gameboy::{
    Flags, Gameboy, OpCode, RegisterLabel8, RegisterLabel16,
    cpu::CPU,
    memory_adapter::MemoryAdapter,
    opcodes::{Argument, Category, JumpCondition},
    tests::decode_util::decode,
    write_flag,
};

#[test]
fn jump_instruction() {
    let instructions = vec![(0x20, false), (0x28, true)];

    for (opcode, condition_val) in instructions {
        let mut gb = Gameboy::new(vec![0x00, 0x00, 0x00, opcode, 0xFB]); // JR NZ -5

        {
            gb.set_register_16(RegisterLabel16::ProgramCounter, 0x0003);
            gb.set_flag(Flags::Z, condition_val);

            let cycles = gb.step_once().unwrap();

            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0000);
            assert_eq!(cycles, 12); // cycles different for action vs no action
        }

        {
            gb.set_register_16(RegisterLabel16::ProgramCounter, 0x0003);
            gb.set_flag(Flags::Z, !condition_val);

            let cycles = gb.step_once().unwrap();

            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0005);
            assert_eq!(cycles, 8);
        }
    }
}

#[test]
fn jr_z_8_instruction_not_working_correctly() {
    let mut gb = Gameboy::new(vec![0x3D, 0x28, 0x08]); // JR Z 8

    // Set the flag as well
    gb.set_flag(Flags::Z, true);

    // Add 0x19 (25) to thee A register
    gb.set_register_8(RegisterLabel8::A, 0x19);

    // DEC A
    gb.step_once();

    println!("A register: {:#X}", gb.get_register_8(RegisterLabel8::A));
    println!("F register: {:#X}", gb.get_register_8(RegisterLabel8::F));

    // The flag should be reset
    assert_eq!(gb.get_flag(Flags::Z), false);

    // Run the jump instruction
    gb.step_once();

    // We should not jump
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x03);
}

#[test]
fn the_jr_with_no_conditions_always_jumps() {
    let mut gb = Gameboy::new(vec![0x00, 0x18, 0x04]);

    // Move the gameboy past the first NOP
    gb.set_register_16(RegisterLabel16::ProgramCounter, 0x01);

    let cycles = gb.step_once().unwrap();
    use crate::gameboy::opcodes::Decoder;
    let opcode = Decoder::decode_instruction(0x01, gb.get_memory_slice_at(0, 0xFFFF)).unwrap();
    assert_eq!(opcode.size(), 2);

    assert_eq!(cycles, 12);
    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x07);
}

#[test]
fn jp_a16_instruction_jumps_to_location() {
    let mut gb = Gameboy::new(vec![0xC3, 0x01, 0x05]); // JP $0501
    let cycles = gb.step_once().unwrap();

    assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x501);
    assert_eq!(cycles, 16);
}

#[test]
fn jp_hl_instruction_jumps() {
    let jump = OpCode::new(
        Category::JP,
        [
            Argument::RegisterIndirect(RegisterLabel16::HL),
            Argument::None,
        ],
    );
    assert_eq!(decode(&[0xE9]), jump);
    assert_eq!(jump.size(), 1);

    let mut cpu = CPU::new();
    let mut memory = vec![0x00, 0xFF];

    cpu.write_16_bits(RegisterLabel16::HL, 0x0123);

    let cycles = jump.run(&mut cpu, MemoryAdapter::new(&mut memory)).unwrap();

    assert_eq!(cycles, 4);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x0123);
}

#[test]
fn decode_jump_tests() {
    assert_eq!(
        decode(&[0xCA, 0x34, 0x12]),
        OpCode::new(
            Category::JP,
            [
                Argument::JumpCondition(JumpCondition::Zero),
                Argument::Label(0x1234)
            ]
        )
    );
    assert_eq!(
        decode(&[0xC2, 0x34, 0x12]),
        OpCode::new(
            Category::JP,
            [
                Argument::JumpCondition(JumpCondition::NotZero),
                Argument::Label(0x1234)
            ]
        )
    );
    assert_eq!(
        decode(&[0xDA, 0x34, 0x12]),
        OpCode::new(
            Category::JP,
            [
                Argument::JumpCondition(JumpCondition::Carry),
                Argument::Label(0x1234)
            ]
        )
    );
    assert_eq!(
        decode(&[0xD2, 0x34, 0x12]),
        OpCode::new(
            Category::JP,
            [
                Argument::JumpCondition(JumpCondition::NotCarry),
                Argument::Label(0x1234)
            ]
        )
    );
    assert_eq!(
        decode(&[0x38, 0x01]),
        OpCode::new(
            Category::JP,
            [
                Argument::JumpCondition(JumpCondition::Carry),
                Argument::JumpDistance(0x01)
            ]
        )
    );
}

#[test]
fn jp_nc_jumps() {
    let opcode = OpCode::new(
        Category::JP,
        [
            Argument::JumpCondition(JumpCondition::NotCarry),
            Argument::Label(0x1234),
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x00, 0xFF];

    write_flag(&mut cpu, Flags::C, false);

    opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01234);

    // Reset & run again
    cpu.write_16_bits(RegisterLabel16::ProgramCounter, 0x0000);
    write_flag(&mut cpu, Flags::C, true);

    opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x0003);
}

#[test]
fn jmp_a16_size_check() {
    let opcode = OpCode::new(
        Category::JP,
        [
            Argument::JumpCondition(JumpCondition::Zero),
            Argument::Label(0x1234),
        ],
    );
    assert_eq!(opcode.size(), 3);
}

#[test]
fn jmp_a16_takes_12_cycles_if_no_jump() {
    let opcode = OpCode::new(
        Category::JP,
        [
            Argument::JumpCondition(JumpCondition::Zero),
            Argument::Label(0x1234),
        ],
    );

    let mut cpu = CPU::new();
    let mut memory = vec![0x00, 0xFF];

    write_flag(&mut cpu, Flags::Z, false);

    let cycles = opcode
        .run(&mut cpu, MemoryAdapter::new(&mut memory))
        .unwrap();

    assert_eq!(cycles, 12);
}
