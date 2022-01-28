#[cfg(test)]
mod dec_test {
    use crate::gameboy::{
        cpu::CPU,
        memory_adapter::MemoryAdapter,
        opcodes::{Argument, Category, Decoder},
        Flags, Gameboy, OpCode, RegisterLabel16, RegisterLabel8,
    };

    fn decode(memory: &[u8]) -> OpCode {
        Decoder::decode_instruction(0x00, memory).unwrap()
    }

    #[test]
    fn dec_instruction_removes_one_from_the_correct_register() {
        let instructions = vec![
            (0x3D, RegisterLabel8::A),
            (0x05, RegisterLabel8::B),
            (0x0D, RegisterLabel8::C),
            (0x15, RegisterLabel8::D),
            (0x1D, RegisterLabel8::E),
            (0x25, RegisterLabel8::H),
            (0x2D, RegisterLabel8::L),
        ];

        for (opcode, register) in instructions {
            let mut gb = Gameboy::new(vec![opcode]);
            gb.set_register_8(register, 6);

            let cycles = gb.step_once();

            // The opcode needs to be 1 byte & take 4 cycles
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1);
            assert_eq!(cycles, 4);

            // The register needs to be decremented
            assert_eq!(gb.get_register_8(register), 5);

            // Test the flags
            // DEC instruction sets the N and zero flag
            assert_eq!(gb.get_flag(Flags::Z), false);
            assert_eq!(gb.get_flag(Flags::N), true);
            assert_eq!(gb.get_flag(Flags::H), false);
        }
    }

    #[test]
    fn dec_hl_offset_can_run() {
        let opcode = OpCode::new(
            Category::DEC,
            [
                Argument::RegisterIndirect(RegisterLabel16::HL),
                Argument::None,
            ],
        );

        let mut cpu = CPU::new();
        let mut memory = vec![0x0; 0xFFFF];

        cpu.write_16_bits(RegisterLabel16::HL, 0x00FF);
        memory[0x00FF] = 0x4;

        let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        assert_eq!(cycles, 12);
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);
        assert_eq!(memory[0x00FF], 0x3);
    }

    #[test]
    fn dec_decode_instructions() {
        assert_eq!(
            decode(&[0x35]),
            OpCode::new(
                Category::DEC,
                [
                    Argument::RegisterIndirect(RegisterLabel16::HL),
                    Argument::None
                ]
            )
        );
    }

    #[test]
    fn dec_sets_the_zero_flag_at_zero() {
        let mut gb = Gameboy::new(vec![0x05]);
        gb.set_register_8(RegisterLabel8::B, 1);
        let _ = gb.step_once();

        assert_eq!(gb.get_flag(Flags::Z), true);
    }

    #[test]
    fn dec_hl_offset_sets_the_zero_flag_at_zero() {
        let mut gb = Gameboy::new(vec![0x35]);
        gb.set_register_16(RegisterLabel16::HL, 0xFF);
        gb.set_memory_at(0xFF, 1);
        let _ = gb.step_once();

        assert_eq!(gb.get_flag(Flags::Z), true);
    }

    #[test]
    fn dec_set_the_h_flag_at_1000() {
        let mut gb = Gameboy::new(vec![0x05]);
        gb.set_register_8(RegisterLabel8::B, 0b1_000);
        let _ = gb.step_once();

        assert_eq!(gb.get_flag(Flags::H), true);
    }

    #[test]
    fn dec_hl_offset_set_the_h_flag_at_1000() {
        let mut gb = Gameboy::new(vec![0x35]);
        gb.set_register_16(RegisterLabel16::HL, 0xFF);
        gb.set_memory_at(0xFF, 0b1_000);
        let _ = gb.step_once();

        assert_eq!(gb.get_flag(Flags::H), true);
    }

    #[test]
    fn dec_should_reset_the_zero_flag_if_already_set() {
        let mut gb = Gameboy::new(vec![0x3D]); // DEC A

        gb.set_register_8(RegisterLabel8::A, 0x19);

        // Set the register
        gb.set_flag(Flags::Z, true);

        // run the instructions
        gb.step_once();

        assert_eq!(gb.get_flag(Flags::Z), false);
    }

    #[test]
    fn dec_hl_offset_should_reset_the_zero_flag_if_already_set() {
        let mut gb = Gameboy::new(vec![0x35]); // DEC (HL)

        gb.set_register_16(RegisterLabel16::HL, 0xFF);
        gb.set_memory_at(0xFF, 0x19);

        // Set the register
        gb.set_flag(Flags::Z, true);

        // run the instructions
        gb.step_once();

        assert_eq!(gb.get_flag(Flags::Z), false);
    }

    #[test]
    fn dec_should_underflow() {
        let mut gb = Gameboy::new(vec![0x15]);
        gb.set_register_8(RegisterLabel8::D, 0);
        let _ = gb.step_once();

        assert_eq!(gb.get_register_8(RegisterLabel8::D), 0xFF);
    }

    #[test]
    fn dec_hl_offset_should_underflow() {
        let mut gb = Gameboy::new(vec![0x35]);
        gb.set_register_16(RegisterLabel16::HL, 0xFF);
        gb.set_memory_at(0xFF, 0);
        let _ = gb.step_once();

        assert_eq!(gb.get_memory_at(0xFF), 0xFF);
    }

    #[test]
    fn dec_with_a_16_bit_register() {
        let opcode = OpCode::new(
            Category::DEC,
            [
                Argument::Register16Constant(RegisterLabel16::BC),
                Argument::None,
            ],
        );

        let mut cpu = CPU::new();
        let mut memory = vec![0x0; 0xFFFF];

        cpu.write_16_bits(RegisterLabel16::BC, 0x04);

        let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        assert_eq!(cpu.read_16_bits(RegisterLabel16::BC), 0x03);
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x1);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn can_decode_16_bit_dec() {
        const BC: RegisterLabel16 = RegisterLabel16::BC;
        const DE: RegisterLabel16 = RegisterLabel16::DE;
        const HL: RegisterLabel16 = RegisterLabel16::HL;
        const SP: RegisterLabel16 = RegisterLabel16::StackPointer;

        assert_eq!(
            decode(&[0x0B]),
            OpCode::new(
                Category::DEC,
                [Argument::Register16Constant(BC), Argument::None]
            )
        );
        assert_eq!(
            decode(&[0x1B]),
            OpCode::new(
                Category::DEC,
                [Argument::Register16Constant(DE), Argument::None]
            )
        );
        assert_eq!(
            decode(&[0x2B]),
            OpCode::new(
                Category::DEC,
                [Argument::Register16Constant(HL), Argument::None]
            )
        );
        assert_eq!(
            decode(&[0x3B]),
            OpCode::new(
                Category::DEC,
                [Argument::Register16Constant(SP), Argument::None]
            )
        );
    }
}
