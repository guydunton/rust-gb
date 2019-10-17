mod opcode_printer_test;
mod pop_test;
mod dec_test;

#[cfg(test)]
mod opcode_tests {
    use crate::gameboy::flags_register::*;
    use crate::gameboy::register::{RegisterLabel16, RegisterLabel8};
    use crate::Gameboy;
    use rust_catch::tests;

    macro_rules! testgb {
        ([ $( $x:expr ),* ]) => {
            Gameboy::new( vec![$($x,)*])
        };
    }

    #[test]
    fn load16_instructions() {
        let ld_16_test = |byte_code, register| {
            let mut gb = testgb!([byte_code, 0xFE, 0xFF]);
            let cycles = gb.step_once();

            assert_eq!(gb.get_register_16(register), 0xFFFE);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0003);
            assert_eq!(cycles, 12);
        };

        // LD SP d16
        ld_16_test(0x31, RegisterLabel16::StackPointer);

        // LD HL d16
        ld_16_test(0x21, RegisterLabel16::HL);

        // LD DE d16
        ld_16_test(0x11, RegisterLabel16::DE);
    }

    #[test]
    fn load8_instructions() {
        {
            // LD (HL-) A
            let mut gb = testgb!([0x32, 0x00]);
            gb.set_register_16(RegisterLabel16::HL, 0x0001);
            gb.set_register_8(RegisterLabel8::A, 0x01);
            let cycles = gb.step_once();

            assert_eq!(gb.get_register_16(RegisterLabel16::HL), 0x0000);
            assert_eq!(gb.get_memory_at(1), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0001);
            assert_eq!(cycles, 8);
        }

        let ld8_test = |byte_code, register| {
            let mut gb = testgb!([byte_code, 0x01]);
            let _ = gb.step_once();
            assert_eq!(gb.get_register_8(register), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);
        };

        // LD c d8
        ld8_test(0x0E, RegisterLabel8::C);

        // LD A d8
        ld8_test(0x3E, RegisterLabel8::A);

        {
            // LD (C) A
            let mut gb = testgb!([0xE2]);
            gb.set_register_8(RegisterLabel8::C, 0x01);
            gb.set_register_8(RegisterLabel8::A, 0x02);

            let cycles = gb.step_once();

            assert_eq!(gb.get_memory_at(0xFF01), 0x02);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
            assert_eq!(cycles, 8);
        }

        {
            // LD (HL) A
            let mut gb = testgb!([0x77]);
            gb.set_register_16(RegisterLabel16::HL, 0x0005);
            gb.set_register_8(RegisterLabel8::A, 0x01);
            let cycles = gb.step_once();

            assert_eq!(gb.get_memory_at(0x0005), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
            assert_eq!(cycles, 8);
        }

        {
            // LDH (a8) A
            let mut gb = testgb!([0xE0, 0x01]);
            gb.set_register_8(RegisterLabel8::A, 0x02);

            let cycles = gb.step_once();
            assert_eq!(gb.get_memory_at(0xFF01) as usize, 0x02);
            assert_eq!(cycles, 12);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);
        }

        {
            // LD A (DE)
            let mut gb = testgb!([0x1A, 0x01]);
            gb.set_register_16(RegisterLabel16::DE, 0x01);

            let cycles = gb.step_once();
            assert_eq!(cycles, 8);
            assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
        }
    }

    #[test]
    fn xor_instruction() {
        let mut gb = testgb!([0xAF]);

        gb.set_register_8(RegisterLabel8::A, 0x01);
        gb.set_register_8(RegisterLabel8::F, 0b1111_0000);

        let cycles = gb.step_once();

        assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x00);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);

        assert_eq!(gb.get_register_8(RegisterLabel8::F), 0x00);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn bit_instruction() {
        // BIT 7,H
        {
            // Check the bit flag when the bit is already set to 1
            let mut gb = testgb!([0xCB, 0x7C]);
            gb.set_register_8(RegisterLabel8::H, 0b1000_0000);
            let carry_flag = gb.get_flag(Flags::C);
            let cycles = gb.step_once();

            assert_eq!(gb.get_flag(Flags::Z), false);
            assert_eq!(gb.get_flag(Flags::N), false);
            assert_eq!(gb.get_flag(Flags::H), true);
            assert_eq!(gb.get_flag(Flags::C), carry_flag); // The carry flag is unaffected

            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x2);
            assert_eq!(cycles, 12);
        }
        {
            // Check the bit flag when the bit is 0
            let mut gb = testgb!([0xCB, 0x7C]);
            gb.set_register_8(RegisterLabel8::H, 0x0);
            let cycles = gb.step_once();

            assert_eq!(gb.get_flag(Flags::Z), true);
            assert_eq!(cycles, 12);
        }
    }

    #[test]
    fn jump_instruction() {
        // JR NZ -5

        let mut gb = testgb!([0x00, 0x00, 0x00, 0x20, 0xFB]);

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

    #[test]
    fn nop_instruction() {
        let mut gb = testgb!([0x00]);
        let cycles = gb.step_once();

        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1);
        assert_eq!(cycles, 4);
    }

    tests! {
        test("increment tests") {
            let mut gb = testgb!([0x0C]); // INC C

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

        test("Call instruction tests") {
            // call 0x0004        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06
            let mut gb = testgb!([0xCD, 0x04, 0x00, 0x03, 0x04, 0x05, 0x06]);
            gb.set_register_16(RegisterLabel16::StackPointer, 0x07);

            /*
            0x00 : instruction
            0x01 : arg part 2
            0x02 : arg part 1
            0x03 : return point and location added to the stack
            0x04 : point where we call to
            0x05 : Part 2 of the stack
            0x06 : part 1 of the stack
            */

            section("Call moves the program counter to the call location") {
                let _ = gb.step_once();
                assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x04);
            }

            section("Call sets the stack value correctly") {
                // We push 0x0003 onto the stack
                // decrements stack and pushed 0x00
                // decrements again and pushed 0x03
                let _ = gb.step_once();
                assert_eq!(gb.get_memory_at(0x05), 0x03);
                assert_eq!(gb.get_memory_at(0x06), 0x00);

                assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0x05);
            }

            section("Call instruction takes 24 cycles") {
                let cycles = gb.step_once();
                assert_eq!(cycles, 24);
            }
        }

        test("Push instruction tests") {

            let mut gb = testgb!([0xC5, 0x00, 0x00]);
            gb.set_register_16(RegisterLabel16::BC, 0x1234);
            gb.set_register_16(RegisterLabel16::StackPointer, 0x03);

            section("Push moves 2 bytes onto the stack") {
                let cycles = gb.step_once();

                assert_eq!(gb.get_memory_at(1), 0x34);
                assert_eq!(gb.get_memory_at(2), 0x12);

                assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0x01);

                // The cycles and the size are correct
                assert_eq!(cycles, 16);
                assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
            }
        }

        test("Rotate left shifts along with the carry flag") {
            let mut gb = testgb!([0xCB, 0x11]);
            // C  C register
            // 1  0101_0101
            // After
            // 0  1010_1011
            gb.set_flag(Flags::C, true);
            gb.set_register_8(RegisterLabel8::C, 0b0101_0101);

            let cycles = gb.step_once();

            assert_eq!(gb.get_register_8(RegisterLabel8::C), 0b1010_1011);
            assert_eq!(gb.get_flag(Flags::C), false);

            assert_eq!(gb.get_flag(Flags::N), false);
            assert_eq!(gb.get_flag(Flags::H), false);
            assert_eq!(gb.get_flag(Flags::Z), false);

            assert_eq!(cycles, 8);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x2);

            // Run again to test the carry flag
            // Before:
            // 0  1010_1011
            // 1  0101_0110
            gb.set_register_16(RegisterLabel16::ProgramCounter, 0x0);

            gb.set_flag(Flags::H, true);
            gb.set_flag(Flags::N, true);

            let _ = gb.step_once();
            assert_eq!(gb.get_flag(Flags::C), true);

            assert_eq!(gb.get_flag(Flags::H), false);
            assert_eq!(gb.get_flag(Flags::N), false);
        }

        test("Rotate left sets the zero flag if the result is 0") {
            let mut gb = testgb!([0xCB, 0x11]);

            let _ = gb.step_once();
            assert_eq!(gb.get_flag(Flags::Z), true);
        }

        test("RLA instruction rotate left through carry") {
            let mut gb = testgb!([0x17]);

            // Before run:
            // C A
            // 1 0101_0101
            // After run:
            // 0 1010_1011
            gb.set_register_8(RegisterLabel8::A, 0b0101_0101);
            gb.set_flag(Flags::C, true);

            section("RLA cycles the A register left through carry") {
                let cycles = gb.step_once();
                assert_eq!(cycles, 4);
                assert_eq!(gb.get_register_8(RegisterLabel8::A), 0b1010_1011);
                assert_eq!(gb.get_flag(Flags::C), false);
            }

            section("sets all flags to false even if they are set") {
                gb.set_flag(Flags::Z, true);
                gb.set_flag(Flags::N, true);
                gb.set_flag(Flags::H, true);

                let _ = gb.step_once();
            }

            assert_eq!(gb.get_flag(Flags::Z), false);
            assert_eq!(gb.get_flag(Flags::N), false);
            assert_eq!(gb.get_flag(Flags::H), false);
        }
    }
}
