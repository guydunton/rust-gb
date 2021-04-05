#[cfg(test)]
mod sub_test {
    use crate::gameboy::Gameboy;
    use crate::gameboy::{Flags, RegisterLabel16, RegisterLabel8};

    #[test]
    fn sub_b_instruction() {
        let mut gb = Gameboy::new(vec![0x90]); // SUB B

        // Set A to greater than B
        gb.set_register_8(RegisterLabel8::A, 5);
        gb.set_register_8(RegisterLabel8::B, 1);

        let cycles = gb.step_once();

        assert_eq!(cycles, 4);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

        assert_eq!(gb.get_register_8(RegisterLabel8::A), 4);

        // The N flag should always be set
        assert_eq!(gb.get_flag(Flags::N), true);

        // Zero should be 0
        assert_eq!(gb.get_flag(Flags::Z), false);

        // H should not be set
        assert_eq!(gb.get_flag(Flags::H), false);

        // C should not be set
        assert_eq!(gb.get_flag(Flags::C), false);
    }

    #[test]
    fn set_the_z_register_if_result_is_zero() {
        let mut gb = Gameboy::new(vec![0x90]);
        gb.set_register_8(RegisterLabel8::A, 3);
        gb.set_register_8(RegisterLabel8::B, 3);

        let _ = gb.step_once();

        assert_eq!(gb.get_flag(Flags::Z), true);
    }

    #[test]
    fn set_c_if_b_greater_than_8() {
        let mut gb = Gameboy::new(vec![0x90]);
        gb.set_register_8(RegisterLabel8::A, 2);
        gb.set_register_8(RegisterLabel8::B, 4);

        let _ = gb.step_once();

        assert_eq!(gb.get_flag(Flags::C), true);
    }

    #[test]
    fn set_h_if_4th_bit_is_borrowed() {
        let mut gb = Gameboy::new(vec![0x90]);
        gb.set_register_8(RegisterLabel8::A, 0b0001_0000);
        gb.set_register_8(RegisterLabel8::B, 1);

        let _ = gb.step_once();

        assert_eq!(gb.get_flag(Flags::H), true);
    }
}
