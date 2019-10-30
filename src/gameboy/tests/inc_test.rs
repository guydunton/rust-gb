#[cfg(test)]
mod inc_test {
    use crate::gameboy::Gameboy;
    use crate::gameboy::{Flags, RegisterLabel16, RegisterLabel8};
    use rust_catch::tests;

    tests! {
        test("increment C tests") {
            let mut gb = Gameboy::new(vec![0x0C]); // INC C

            section("increment increases the value in the registry") {
                let cycles = gb.step_once();

                assert_eq!(gb.get_register_8(RegisterLabel8::C), 0x01);
                assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

                assert_eq!(cycles, 4);
            }

            section("increment can cause a half overflow") {
                gb.set_register_8(RegisterLabel8::C, 0b1111);
                gb.step_once();

                assert_eq!(gb.get_flag(Flags::H), true);
            }

            section("increment from max causes overflow") {
                gb.set_register_8(RegisterLabel8::C, 0xFF);
                gb.step_once();

                assert_eq!(gb.get_register_8(RegisterLabel8::C), 0x0);
                assert_eq!(gb.get_flag(Flags::Z), true);
            }

            section("increment doesnt reset flags set flags") {
                // Increment doesn't reset the Z and H if they are already set
                gb.set_flag(Flags::Z, true);
                gb.set_flag(Flags::H, true);
                gb.set_register_8(RegisterLabel8::C, 0x01);

                gb.step_once();

                assert_eq!(gb.get_flag(Flags::Z), true);
                assert_eq!(gb.get_flag(Flags::H), true);
            }

            section("N flag is set to 0") {
                gb.set_flag(Flags::N, true);
                gb.step_once();

                assert_eq!(gb.get_flag(Flags::N), false);
            }
        }

        test("increment B tests") {
            let mut gb = Gameboy::new(vec![0x04]); // INC B

            section("increment increases the value in the registry") {
                let cycles = gb.step_once();

                assert_eq!(gb.get_register_8(RegisterLabel8::B), 0x01);
                assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

                assert_eq!(cycles, 4);
            }

            section("increment can cause a half overflow") {
                gb.set_register_8(RegisterLabel8::B, 0b1111);
                gb.step_once();

                assert_eq!(gb.get_flag(Flags::H), true);
            }

            section("increment from max causes overflow") {
                gb.set_register_8(RegisterLabel8::B, 0xFF);
                gb.step_once();

                assert_eq!(gb.get_register_8(RegisterLabel8::B), 0x0);
                assert_eq!(gb.get_flag(Flags::Z), true);
            }

            section("increment doesnt reset flags set flags") {
                // Increment doesn't reset the Z and H if they are already set
                gb.set_flag(Flags::Z, true);
                gb.set_flag(Flags::H, true);
                gb.set_register_8(RegisterLabel8::B, 0x01);

                gb.step_once();

                assert_eq!(gb.get_flag(Flags::Z), true);
                assert_eq!(gb.get_flag(Flags::H), true);
            }

            section("N flag is set to 0") {
                gb.set_flag(Flags::N, true);
                gb.step_once();

                assert_eq!(gb.get_flag(Flags::N), false);
            }
        }


        test("INC 16 instruction") {

            // INC HL
            // INC DE
            let instructions: Vec<(u8, RegisterLabel16)> = vec![
                (0x23, RegisterLabel16::HL),
                (0x13, RegisterLabel16::DE)
            ];

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
}
