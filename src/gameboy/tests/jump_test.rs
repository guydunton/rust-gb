#[cfg(test)]
mod jump_test {
    use crate::gameboy::{Flags, Gameboy, RegisterLabel16};
    use rust_catch::tests;

    #[test]
    fn jump_instruction() {
        // JR NZ -5

        let mut gb = Gameboy::new(vec![0x00, 0x00, 0x00, 0x20, 0xFB]);

        {
            gb.set_register_16(RegisterLabel16::ProgramCounter, 0x0003);
            gb.set_flag(Flags::Z, false);

            let cycles = gb.step_once();

            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0000);
            assert_eq!(cycles, 12); // cycles different for action vs no action
        }

        {
            gb.set_register_16(RegisterLabel16::ProgramCounter, 0x0003);
            gb.set_flag(Flags::Z, true);

            let cycles = gb.step_once();

            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0005);
            assert_eq!(cycles, 8);
        }
    }
}
