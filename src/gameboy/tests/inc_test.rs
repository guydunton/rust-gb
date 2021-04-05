#[cfg(test)]
mod inc_test {

    mod increment_c_tests {

        use crate::gameboy::Gameboy;
        use crate::gameboy::{Flags, RegisterLabel16, RegisterLabel8};

        #[test]
        fn increment_increases_the_value_in_the_registry() {
            let mut gb = Gameboy::new(vec![0x0C]); // INC C
            let cycles = gb.step_once();

            assert_eq!(gb.get_register_8(RegisterLabel8::C), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

            assert_eq!(cycles, 4);
        }

        #[test]
        fn increment_can_cause_a_half_overflow() {
            let mut gb = Gameboy::new(vec![0x0C]); // INC C
            gb.set_register_8(RegisterLabel8::C, 0b1111);
            gb.step_once();

            assert_eq!(gb.get_flag(Flags::H), true);
        }

        #[test]
        fn increment_from_max_causes_overflow() {
            let mut gb = Gameboy::new(vec![0x0C]); // INC C
            gb.set_register_8(RegisterLabel8::C, 0xFF);
            gb.step_once();

            assert_eq!(gb.get_register_8(RegisterLabel8::C), 0x0);
            assert_eq!(gb.get_flag(Flags::Z), true);
        }

        #[test]
        fn increment_doesnt_reset_flags_set_flags() {
            let mut gb = Gameboy::new(vec![0x0C]); // INC C
                                                   // Increment doesn't reset the Z and H if they are already set
            gb.set_flag(Flags::Z, true);
            gb.set_flag(Flags::H, true);
            gb.set_register_8(RegisterLabel8::C, 0x01);

            gb.step_once();

            assert_eq!(gb.get_flag(Flags::Z), true);
            assert_eq!(gb.get_flag(Flags::H), true);
        }

        #[test]
        fn n_flag_is_set_to_0() {
            let mut gb = Gameboy::new(vec![0x0C]); // INC C
            gb.set_flag(Flags::N, true);
            gb.step_once();

            assert_eq!(gb.get_flag(Flags::N), false);
        }
    }

    mod increment_b_tests {

        use crate::gameboy::Gameboy;
        use crate::gameboy::{Flags, RegisterLabel16, RegisterLabel8};

        #[test]
        fn increment_increases_the_value_in_the_registry() {
            let mut gb = Gameboy::new(vec![0x04]); // INC B
            let cycles = gb.step_once();

            assert_eq!(gb.get_register_8(RegisterLabel8::B), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

            assert_eq!(cycles, 4);
        }

        #[test]
        fn increment_can_cause_a_half_overflow() {
            let mut gb = Gameboy::new(vec![0x04]); // INC B
            gb.set_register_8(RegisterLabel8::B, 0b1111);
            gb.step_once();

            assert_eq!(gb.get_flag(Flags::H), true);
        }

        #[test]
        fn increment_from_max_causes_overflow() {
            let mut gb = Gameboy::new(vec![0x04]); // INC B
            gb.set_register_8(RegisterLabel8::B, 0xFF);
            gb.step_once();

            assert_eq!(gb.get_register_8(RegisterLabel8::B), 0x0);
            assert_eq!(gb.get_flag(Flags::Z), true);
        }

        #[test]
        fn increment_doesnt_reset_flags_set_flags() {
            let mut gb = Gameboy::new(vec![0x04]); // INC B
                                                   // Increment doesn't reset the Z and H if they are already set
            gb.set_flag(Flags::Z, true);
            gb.set_flag(Flags::H, true);
            gb.set_register_8(RegisterLabel8::B, 0x01);

            gb.step_once();

            assert_eq!(gb.get_flag(Flags::Z), true);
            assert_eq!(gb.get_flag(Flags::H), true);
        }

        #[test]
        fn n_flag_is_set_to_0() {
            let mut gb = Gameboy::new(vec![0x04]); // INC B
            gb.set_flag(Flags::N, true);
            gb.step_once();

            assert_eq!(gb.get_flag(Flags::N), false);
        }
    }

    mod increment_h_tests {

        use crate::gameboy::Gameboy;
        use crate::gameboy::{Flags, RegisterLabel16, RegisterLabel8};

        #[test]
        fn increment_increases_the_value_in_the_registry() {
            let mut gb = Gameboy::new(vec![0x24]); // INC H
            let cycles = gb.step_once();

            assert_eq!(gb.get_register_8(RegisterLabel8::H), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

            assert_eq!(cycles, 4);
        }

        #[test]
        fn increment_can_cause_a_half_overflow() {
            let mut gb = Gameboy::new(vec![0x24]); // INC H
            gb.set_register_8(RegisterLabel8::H, 0b1111);
            gb.step_once();

            assert_eq!(gb.get_flag(Flags::H), true);
        }

        #[test]
        fn increment_from_max_causes_overflow() {
            let mut gb = Gameboy::new(vec![0x24]); // INC H
            gb.set_register_8(RegisterLabel8::H, 0xFF);
            gb.step_once();

            assert_eq!(gb.get_register_8(RegisterLabel8::H), 0x0);
            assert_eq!(gb.get_flag(Flags::Z), true);
        }

        #[test]
        fn increment_doesnt_reset_flags_set_flags() {
            let mut gb = Gameboy::new(vec![0x24]); // INC H
                                                   // Increment doesn't reset the Z and H if they are already set
            gb.set_flag(Flags::Z, true);
            gb.set_flag(Flags::H, true);
            gb.set_register_8(RegisterLabel8::H, 0x01);

            gb.step_once();

            assert_eq!(gb.get_flag(Flags::Z), true);
            assert_eq!(gb.get_flag(Flags::H), true);
        }

        #[test]
        fn n_flag_is_set_to_0() {
            let mut gb = Gameboy::new(vec![0x24]); // INC H
            gb.set_flag(Flags::N, true);
            gb.step_once();

            assert_eq!(gb.get_flag(Flags::N), false);
        }
    }

    #[test]
    fn inc_16_instruction() {
        use crate::gameboy::Gameboy;
        use crate::gameboy::{Flags, RegisterLabel16};

        // INC HL
        // INC DE
        let instructions: Vec<(u8, RegisterLabel16)> =
            vec![(0x23, RegisterLabel16::HL), (0x13, RegisterLabel16::DE)];

        for &(instruction, register) in instructions.iter() {
            let mut gb = Gameboy::new(vec![instruction]);
            let cycles = gb.step_once();

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
}
