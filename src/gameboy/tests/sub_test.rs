#[cfg(test)]
mod sub_test {
    use crate::gameboy::Gameboy;
    use crate::gameboy::{Flags, RegisterLabel16, RegisterLabel8};
    use rust_catch::tests;

    tests! {
        test("SUB B instruction") {
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

        test("SUB flags tests") {
            let mut gb = Gameboy::new(vec![0x90]);

            section("set the Z register if result is zero") {
                gb.set_register_8(RegisterLabel8::A, 3);
                gb.set_register_8(RegisterLabel8::B, 3);

                let _ = gb.step_once();

                assert_eq!(gb.get_flag(Flags::Z), true);
            }

            section("set C if B greater than 8") {
                gb.set_register_8(RegisterLabel8::A, 2);
                gb.set_register_8(RegisterLabel8::B, 4);

                let _ = gb.step_once();

                assert_eq!(gb.get_flag(Flags::C), true);
            }
        }
    }
}
