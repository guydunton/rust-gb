#[cfg(test)]
mod load16_test {
    use crate::gameboy::Gameboy;
    use crate::gameboy::RegisterLabel16;
    use rust_catch::tests;

    fn load16_instructions(byte_code: u8) -> (Gameboy<'static>, u32) {
        let mut gb = Gameboy::new(vec![byte_code, 0xFE, 0xFF]);
        let cycles = gb.step_once();
        (gb, cycles)
    }

    tests! {
        test("Test the load16 instructions") {
            section("LD SP d16") {
                let (gb, cycles) = load16_instructions(0x31);
                assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0xFFFE);
                assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0003);
                assert_eq!(cycles, 12);
            }

            section("LD HL d16") {
                let (gb, cycles) = load16_instructions(0x21);
                assert_eq!(gb.get_register_16(RegisterLabel16::HL), 0xFFFE);
                assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0003);
                assert_eq!(cycles, 12);
            }

            section("LD DE d16") {
                let (gb, cycles) = load16_instructions(0x11);
                assert_eq!(gb.get_register_16(RegisterLabel16::DE), 0xFFFE);
                assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0003);
                assert_eq!(cycles, 12);
            }
        }
    }
}
