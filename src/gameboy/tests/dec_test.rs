#[cfg(test)]
mod dec_test {
    use crate::gameboy::{Flags, Gameboy, RegisterLabel16, RegisterLabel8};
    use rust_catch::tests;

    tests! {
        test("DEC instruction removes one from the correct register") {
            let mut gb = Gameboy::new(vec![0x05]);
            gb.set_register_8(RegisterLabel8::B, 6);

            let cycles = gb.step_once();

            // The opcode needs to be 1 byte & take 4 cycles
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1);
            assert_eq!(cycles, 4);

            // The register needs to be decremented
            assert_eq!(gb.get_register_8(RegisterLabel8::B), 5);

            // Test the flags
            section("DEC instruction sets the N and zero flag") {
                assert_eq!(gb.get_flag(Flags::Z), false);
                assert_eq!(gb.get_flag(Flags::N), true);
                assert_eq!(gb.get_flag(Flags::H), false);
            }
        }

        test("DEC sets the zero flag at zero") {
            let mut gb = Gameboy::new(vec![0x05]);
            gb.set_register_8(RegisterLabel8::B, 1);
            let _ = gb.step_once();

            assert_eq!(gb.get_flag(Flags::Z), true);
        }

        test("DEC set the H flag at 1000") {
            let mut gb = Gameboy::new(vec![0x05]);
            gb.set_register_8(RegisterLabel8::B, 0b1_000);
            let _ = gb.step_once();

            assert_eq!(gb.get_flag(Flags::H), true);
        }
    }
}
