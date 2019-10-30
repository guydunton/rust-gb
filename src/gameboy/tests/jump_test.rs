#[cfg(test)]
mod jump_test {
    use crate::gameboy::{Flags, Gameboy, RegisterLabel16};

    #[test]
    fn jump_instruction() {
        let instructions = vec![(0x20, false), (0x28, true)];

        for (opcode, condition_val) in instructions {
            // JR NZ -5
            let mut gb = Gameboy::new(vec![0x00, 0x00, 0x00, opcode, 0xFB]);

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
}
