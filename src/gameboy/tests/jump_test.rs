#[cfg(test)]
mod jump_test {
    use crate::gameboy::{Flags, Gameboy, RegisterLabel16, RegisterLabel8};

    #[test]
    fn jump_instruction() {
        let instructions = vec![(0x20, false), (0x28, true)];

        for (opcode, condition_val) in instructions {
            let mut gb = Gameboy::new(vec![0x00, 0x00, 0x00, opcode, 0xFB]); // JR NZ -5

            {
                gb.set_register_16(RegisterLabel16::ProgramCounter, 0x0003);
                gb.set_flag(Flags::Z, condition_val);

                let cycles = gb.step_once();

                assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0000);
                assert_eq!(cycles, 12); // cycles different for action vs no action
            }

            {
                gb.set_register_16(RegisterLabel16::ProgramCounter, 0x0003);
                gb.set_flag(Flags::Z, !condition_val);

                let cycles = gb.step_once();

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

        let cycles = gb.step_once();
        use super::super::super::opcodes::decode_instruction;
        let opcode = decode_instruction(0x01, gb.get_memory_slice_at(0, 0xFFFF)).unwrap();
        assert_eq!(opcode.size(), 2);

        assert_eq!(cycles, 12);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x07);
    }

    #[test]
    fn jp_a16_instruction_jumps_to_location() {
        let mut gb = Gameboy::new(vec![0xC3, 0x01, 0x05]); // JP $0501
        let cycles = gb.step_once();

        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x501);
        assert_eq!(cycles, 16);
    }
}
