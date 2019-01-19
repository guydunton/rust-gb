#[cfg(test)]
mod opcode_tests {

    use crate::gameboy::cpu::CPU;
    use crate::gameboy::flags_register::*;
    use crate::gameboy::opcode_library::{decode_instruction, OpCode};
    use crate::gameboy::read_write_register::ReadWriteRegister;
    use crate::gameboy::register::{RegisterLabel16, RegisterLabel8};
    use rust_catch::test_suite;

    struct TestGB {
        cpu: CPU,
        memory: Vec<u8>,
    }

    impl TestGB {
        fn new(data: Vec<u8>) -> TestGB {
            let mut memory = vec![0; 0xFFFF];
            memory[..data.len()].clone_from_slice(&data[..]);

            TestGB {
                cpu: CPU::new(),
                memory,
            }
        }

        fn decode(&self) -> OpCode {
            decode_instruction(
                self.cpu.read_16_bits(RegisterLabel16::ProgramCounter),
                &self.memory,
            )
            .unwrap()
        }

        fn run(&mut self, opcode: &OpCode) -> u32 {
            opcode.run::<CPU>(&mut self.cpu, &mut self.memory)
        }

        fn decode_and_run(&mut self) -> u32 {
            let opcode = self.decode();
            self.run(&opcode)
        }

        fn set_flag(&mut self, flag: Flags, val: bool) {
            write_flag::<CPU>(&mut self.cpu, flag, val);
        }

        fn get_flag(&self, flag: Flags) -> bool {
            read_flag::<CPU>(&self.cpu, flag)
        }

        fn write_16(&mut self, register: RegisterLabel16, val: u16) {
            self.cpu.write_16_bits(register, val);
        }

        fn read_16(&self, register: RegisterLabel16) -> u16 {
            self.cpu.read_16_bits(register)
        }

        fn write_8(&mut self, register: RegisterLabel8, val: u8) {
            self.cpu.write_8_bits(register, val);
        }

        fn read_8(&self, register: RegisterLabel8) -> u8 {
            self.cpu.read_8_bits(register)
        }
    }

    macro_rules! testgb {
        ([ $( $x:expr ),* ]) => {
            TestGB::new( vec![$($x,)*])
        };
    }

    #[test]
    fn load16_instructions() {
        let ld_16_test = |byte_code, register| {
            let mut gb = testgb!([byte_code, 0xFE, 0xFF]);
            let cycles = gb.decode_and_run();

            assert_eq!(gb.read_16(register), 0xFFFE);
            assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x0003);
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
            gb.write_16(RegisterLabel16::HL, 0x0001);
            gb.write_8(RegisterLabel8::A, 0x01);
            let cycles = gb.decode_and_run();

            assert_eq!(gb.read_16(RegisterLabel16::HL), 0x0000);
            assert_eq!(gb.memory[1], 0x01);
            assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x0001);
            assert_eq!(cycles, 8);
        }

        let ld8_test = |byte_code, register| {
            let mut gb = testgb!([byte_code, 0x01]);
            let _ = gb.decode_and_run();
            assert_eq!(gb.read_8(register), 0x01);
            assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x02);
        };

        // LD c d8
        ld8_test(0x0E, RegisterLabel8::C);

        // LD A d8
        ld8_test(0x3E, RegisterLabel8::A);

        {
            // LD (C) A
            let mut gb = testgb!([0xE2]);
            gb.write_8(RegisterLabel8::C, 0x01);
            gb.write_8(RegisterLabel8::A, 0x02);

            let cycles = gb.decode_and_run();

            assert_eq!(gb.memory[0xFF01], 0x02);
            assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x01);
            assert_eq!(cycles, 8);
        }

        {
            // LD (HL) A
            let mut gb = testgb!([0x77]);
            gb.write_16(RegisterLabel16::HL, 0x0005);
            gb.write_8(RegisterLabel8::A, 0x01);
            let cycles = gb.decode_and_run();

            assert_eq!(gb.memory[0x0005], 0x01);
            assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x01);
            assert_eq!(cycles, 8);
        }

        {
            // LDH (a8) A
            let mut gb = testgb!([0xE0, 0x01]);
            gb.write_8(RegisterLabel8::A, 0x02);

            let cycles = gb.decode_and_run();
            assert_eq!(gb.memory[0xFF01 as usize], 0x02);
            assert_eq!(cycles, 12);
            assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x02);
        }

        {
            // LD A (DE)
            let mut gb = testgb!([0x1A, 0x01]);
            gb.write_16(RegisterLabel16::DE, 0x01);

            let cycles = gb.decode_and_run();
            assert_eq!(cycles, 8);
            assert_eq!(gb.read_8(RegisterLabel8::A), 0x01);
            assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x01);
        }
    }

    #[test]
    fn xor_instruction() {
        let mut gb = testgb!([0xAF]);

        gb.write_8(RegisterLabel8::A, 0x01);
        gb.write_8(RegisterLabel8::F, 0b1111_0000);

        let cycles = gb.decode_and_run();

        assert_eq!(gb.read_8(RegisterLabel8::A), 0x00);
        assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x01);

        assert_eq!(gb.read_8(RegisterLabel8::F), 0x00);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn bit_instruction() {
        // BIT 7,H
        {
            // Check the bit flag when the bit is already set to 1
            let mut gb = testgb!([0xCB, 0x7C]);
            gb.write_8(RegisterLabel8::H, 0b1000_0000);
            let carry_flag = gb.get_flag(Flags::C);
            let cycles = gb.decode_and_run();

            assert_eq!(gb.get_flag(Flags::Z), false);
            assert_eq!(gb.get_flag(Flags::N), false);
            assert_eq!(gb.get_flag(Flags::H), true);
            assert_eq!(gb.get_flag(Flags::C), carry_flag); // The carry flag is unaffected

            assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x2);
            assert_eq!(cycles, 12);
        }
        {
            // Check the bit flag when the bit is 0
            let mut gb = testgb!([0xCB, 0x7C]);
            gb.write_8(RegisterLabel8::H, 0x0);
            let cycles = gb.decode_and_run();

            assert_eq!(gb.get_flag(Flags::Z), true);
            assert_eq!(cycles, 12);
        }
    }

    #[test]
    fn jump_instruction() {
        // JR NZ -5

        let mut gb = TestGB::new(vec![0x00, 0x00, 0x00, 0x20, 0xFB]);

        {
            gb.write_16(RegisterLabel16::ProgramCounter, 0x0003);
            gb.set_flag(Flags::Z, false);

            let opcode = gb.decode();
            let cycles = gb.run(&opcode);

            assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x0000);
            assert_eq!(cycles, 12); // cycles different for action vs no action
        }

        {
            gb.write_16(RegisterLabel16::ProgramCounter, 0x0003);
            gb.set_flag(Flags::Z, true);

            let opcode = gb.decode();
            let cycles = gb.run(&opcode);

            assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x0005);
            assert_eq!(cycles, 8);
        }
    }

    #[test]
    fn nop_instruction() {
        let mut gb = testgb!([0x00]);
        let cycles = gb.decode_and_run();

        assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x1);
        assert_eq!(cycles, 4);
    }

    test_suite! {
        test_case("increment tests") {
            let mut gb = testgb!([0x0C]); // INC C

            section("increment increases the value in the registry") {
                let cycles = gb.decode_and_run();

                assert_eq!(gb.read_8(RegisterLabel8::C), 0x01);
                assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x01);

                assert_eq!(cycles, 4);
            }

            section("increment can cause a half overflow") {
                gb.write_8(RegisterLabel8::C, 0b1111);
                gb.decode_and_run();

                assert_eq!(gb.get_flag(Flags::H), true);
            }

            section("increment from max causes overflow") {
                gb.write_8(RegisterLabel8::C, 0xFF);
                gb.decode_and_run();

                assert_eq!(gb.read_8(RegisterLabel8::C), 0x0);
                assert_eq!(gb.get_flag(Flags::Z), true);
            }

            section("increment doesnt reset flags set flags") {
                // Increment doesn't reset the Z and H if they are already set
                gb.set_flag(Flags::Z, true);
                gb.set_flag(Flags::H, true);
                gb.write_8(RegisterLabel8::C, 0x01);

                gb.decode_and_run();

                assert_eq!(gb.get_flag(Flags::Z), true);
                assert_eq!(gb.get_flag(Flags::H), true);
            }

            section("N flag is set to 0") {
                gb.set_flag(Flags::N, true);
                gb.decode_and_run();

                assert_eq!(gb.get_flag(Flags::N), false);
            }
        }

        test_case("Call instruction tests") {
            // call 0x0004        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06
            let mut gb = testgb!([0xCD, 0x04, 0x00, 0x03, 0x04, 0x05, 0x06]);
            gb.write_16(RegisterLabel16::StackPointer, 0x07);

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
                let _ = gb.decode_and_run();
                assert_eq!(gb.read_16(RegisterLabel16::ProgramCounter), 0x04);
            }

            section("Call sets the stack value correctly") {
                // We push 0x0003 onto the stack
                // decrements stack and pushed 0x00
                // decrements again and pushed 0x03
                let _ = gb.decode_and_run();
                assert_eq!(gb.memory[0x05], 0x03);
                assert_eq!(gb.memory[0x06], 0x00);

                assert_eq!(gb.read_16(RegisterLabel16::StackPointer), 0x05);
            }

            section("Call instruction is 3 bytes and 24 cycles") {
                let opcode = gb.decode();
                let cycles = gb.run(&opcode);
                assert_eq!(cycles, 24);
                assert_eq!(opcode.size(), 3);
            }
        }
    }

}
