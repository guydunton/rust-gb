#[cfg(test)]
mod dec_test {
    use crate::gameboy::{Flags, Gameboy, RegisterLabel16, RegisterLabel8};

    #[test]
    fn dec_instruction_removes_one_from_the_correct_register() {
        let instructions = vec![
            (0x3D, RegisterLabel8::A),
            (0x05, RegisterLabel8::B),
            (0x0D, RegisterLabel8::C),
            (0x15, RegisterLabel8::D),
            (0x1D, RegisterLabel8::E),
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
    fn dec_sets_the_zero_flag_at_zero() {
        let mut gb = Gameboy::new(vec![0x05]);
        gb.set_register_8(RegisterLabel8::B, 1);
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
    fn dec_should_underflow() {
        let mut gb = Gameboy::new(vec![0x15]);
        gb.set_register_8(RegisterLabel8::D, 0);
        let _ = gb.step_once();

        assert_eq!(gb.get_register_8(RegisterLabel8::D), 0xFF);
    }

    #[test]
    fn dec_also_works_with_16_bit_registers() {
        let instructions = vec![(0x0Bu8, RegisterLabel16::BC)];

        for (opcode, register) in instructions {
            let mut gb = Gameboy::new(vec![opcode]);
            gb.set_register_16(register, 6);

            let cycles = gb.step_once();

            // The opcode needs to be 1 byte & take 4 cycles
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1);
            assert_eq!(cycles, 8);

            // The register needs to be decremented
            assert_eq!(gb.get_register_16(register), 5);
        }
    }
}
