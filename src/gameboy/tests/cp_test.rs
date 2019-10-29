#[cfg(test)]
mod cp_test {
    use crate::gameboy::{Flags, Gameboy, RegisterLabel16, RegisterLabel8};
    use rust_catch::tests;

    tests! {
        test("CP instruction leaves A unchanged") {
            // CP removes the value from the A register but throws away the result

            let mut gb = Gameboy::new(vec![0xFE, 012]);
            gb.set_register_8(RegisterLabel8::A, 0x03);

            let cycles = gb.step_once();

            assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x03);

            // The size and cycles are correct
            assert_eq!(cycles, 8);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);
        }

        test("Flag tests") {
            let mut gb = Gameboy::new(vec![0xFE, 0x03]);

            section("Z flag is set if result is 0") {
                gb.set_register_8(RegisterLabel8::A, 0x03);
                let _ = gb.step_once();

                assert_eq!(gb.get_flag(Flags::Z), true);
            }

            section("Set the C flag if the value is greater than 0") {
                gb.set_register_8(RegisterLabel8::A, 0x01);
                gb.step_once();
                assert_eq!(gb.get_flag(Flags::C), true);
            }

            section("N flag must be set") {
                let _ = gb.step_once();
                assert_eq!(gb.get_flag(Flags::N), true);
            }

            section("H flag is set correctly") {
                gb.set_register_8(RegisterLabel8::A, 0b0001_0000);
                let _ = gb.step_once();
                assert_eq!(gb.get_flag(Flags::H), true);
            }
        }

    }
}
