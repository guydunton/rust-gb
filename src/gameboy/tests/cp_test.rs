#[cfg(test)]
mod cp_test {
    use crate::gameboy::{Flags, Gameboy, RegisterLabel16, RegisterLabel8};

    struct CPFixture<'a> {
        gb: Gameboy<'a>,
    }

    impl<'a> CPFixture<'a> {
        fn setup(memory: Vec<u8>) -> CPFixture<'a> {
            CPFixture {
                gb: Gameboy::new(memory),
            }
        }

        fn set_source(mut self, val: u8) -> Self {
            match self.gb.get_memory_at(0x0000) {
                0xBE => {
                    self.gb.set_register_16(RegisterLabel16::HL, 0x4000);
                    self.gb.set_memory_at(0x4000, val);
                }
                _ => {
                    panic!("Test not setup");
                }
            }
            self
        }

        fn set_a(mut self, val: u8) -> Self {
            self.gb.set_register_8(RegisterLabel8::A, val);
            self
        }

        fn step(&mut self) -> u32 {
            self.gb.step_once().unwrap()
        }
    }

    #[test]
    fn cp_instruction_leaves_a_unchanged() {
        // CP removes the value from the A register but throws away the result
        let mut f = CPFixture::setup(vec![0xFE, 0x12]).set_a(0x03);
        let cycles = f.step();

        assert_eq!(f.gb.get_register_8(RegisterLabel8::A), 0x03);

        // The size and cycles are correct
        assert_eq!(cycles, 8);
        assert_eq!(f.gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);
    }

    #[test]
    fn flag_tests_z_flag_is_set_if_result_is_0() {
        let mut f = CPFixture::setup(vec![0xFE, 0x03]).set_a(0x03);
        f.step();
        assert!(f.gb.get_flag(Flags::Z));
    }

    #[test]
    fn flag_tests_set_the_c_flag_if_the_result_is_less_than_0() {
        let mut f = CPFixture::setup(vec![0xFE, 0x03]).set_a(0x01);
        f.step();
        assert!(f.gb.get_flag(Flags::C));
    }

    #[test]
    fn flag_tests_n_flag_must_be_set() {
        let mut f = CPFixture::setup(vec![0xFE, 0x03]);
        f.step();
        assert!(f.gb.get_flag(Flags::N));
    }

    #[test]
    fn flag_tests_h_flag_is_set_correctly() {
        let mut f = CPFixture::setup(vec![0xFE, 0x03]).set_a(0b0001_0000);
        f.step();
        assert!(f.gb.get_flag(Flags::H));
    }

    #[test]
    fn next_cp_instruction_tests() {
        let mut f = CPFixture::setup(vec![0xBE])
            .set_source(0x03) // Set HL val
            .set_a(0x01);

        let cycles = f.step();

        assert_eq!(f.gb.get_register_8(RegisterLabel8::A), 0x01);

        // The size and cycles are correct
        assert_eq!(cycles, 8);
        assert_eq!(f.gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

        // N Flag should be set
        assert_eq!(f.gb.get_flag(Flags::N), true);
    }

    #[test]
    fn cp_hl_carry_if_result_negative() {
        let mut f = CPFixture::setup(vec![0xBE]).set_a(0x01).set_source(0x03);
        f.step();
        assert!(f.gb.get_flag(Flags::C));
    }

    #[test]
    fn cp_hl_z_if_result_zero() {
        let mut f = CPFixture::setup(vec![0xBE]).set_a(0x3).set_source(0x3);
        f.step();
        assert!(f.gb.get_flag(Flags::Z));
    }
}
