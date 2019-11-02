#[cfg(test)]
mod load8_test {
    use crate::gameboy::Gameboy;
    use crate::gameboy::{RegisterLabel16, RegisterLabel8};
    use rust_catch::tests;

    #[test]
    fn load8_instructions() {
        {
            // LD (HL-) A
            let mut gb = Gameboy::new(vec![0x32, 0x00]);
            gb.set_register_16(RegisterLabel16::HL, 0x0001);
            gb.set_register_8(RegisterLabel8::A, 0x01);
            let cycles = gb.step_once();

            assert_eq!(gb.get_register_16(RegisterLabel16::HL), 0x0000);
            assert_eq!(gb.get_memory_at(1), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x0001);
            assert_eq!(cycles, 8);
        }

        let ld8_test = |byte_code, register| {
            let mut gb = Gameboy::new(vec![byte_code, 0x01]);
            let _ = gb.step_once();
            assert_eq!(gb.get_register_8(register), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);
        };

        // LD c d8
        ld8_test(0x0E, RegisterLabel8::C);

        // LD A d8
        ld8_test(0x3E, RegisterLabel8::A);

        ld8_test(0x1E, RegisterLabel8::E);

        {
            // LD C A
            let mut gb = Gameboy::new(vec![0xE2]);
            gb.set_register_8(RegisterLabel8::C, 0x01);
            gb.set_register_8(RegisterLabel8::A, 0x02);

            let cycles = gb.step_once();

            assert_eq!(gb.get_memory_at(0xFF01), 0x02);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
            assert_eq!(cycles, 8);
        }

        {
            // LD (HL) A
            let mut gb = Gameboy::new(vec![0x77]);
            gb.set_register_16(RegisterLabel16::HL, 0x0005);
            gb.set_register_8(RegisterLabel8::A, 0x01);
            let cycles = gb.step_once();

            assert_eq!(gb.get_memory_at(0x0005), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
            assert_eq!(cycles, 8);
        }

        {
            // LD A (DE)
            let mut gb = Gameboy::new(vec![0x1A, 0x01]);
            gb.set_register_16(RegisterLabel16::DE, 0x01);

            let cycles = gb.step_once();
            assert_eq!(cycles, 8);
            assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
        }
    }

    tests! {
        test("LD8 HL plus A") {
            // LD (HL+), A
            let mut gb = Gameboy::new(vec![0x22, 0x00]);
            gb.set_register_16(RegisterLabel16::HL, 0x0001);
            gb.set_register_8(RegisterLabel8::A, 0x12);

            let cycles = gb.step_once();

            assert_eq!(cycles, 8);
            assert_eq!(gb.get_register_16(RegisterLabel16::HL), 0x02);
            assert_eq!(gb.get_memory_at(0x01), 0x12);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
        }

        test("LDH a8 A") {

            // LDH (a8) A
            let mut gb = Gameboy::new(vec![0xE0, 0x01]);
            gb.set_register_8(RegisterLabel8::A, 0x02);

            let cycles = gb.step_once();

            assert_eq!(gb.get_memory_at(0xFF01) as usize, 0x02);
            assert_eq!(cycles, 12);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);
        }

        test("LDH A a8") {
            let mut gb = Gameboy::new(vec![0xF0, 0x02]);
            gb.set_memory_at(0xFF02, 0x34);

            let cycles = gb.step_once();

            println!("A register: {:?}", gb.get_register_8(RegisterLabel8::A));

            assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x34);
            assert_eq!(cycles, 12);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);
        }

        test("Generic LD8 r8 r8 test") {

            let instructions = vec![
                (0x7B, RegisterLabel8::A, RegisterLabel8::E),
                (0x67, RegisterLabel8::H, RegisterLabel8::A),
                (0x57, RegisterLabel8::D, RegisterLabel8::A),
            ];

            for &(code, dest, src) in instructions.iter() {
                let mut gb= Gameboy::new(vec![code]);

                gb.set_register_8(src, 0x04);
                gb.set_register_8(dest, 0x01);

                let _ = gb.step_once();

                assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x04);
            }
        }

        test("LD8 into address address") {
            let mut gb = Gameboy::new(vec![0xEA, 0x10, 0x99]); // LD8 ($9910), A

            gb.set_register_8(RegisterLabel8::A, 0xFF);

            let cycles = gb.step_once();

            assert_eq!(cycles, 16);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x03);
            assert_eq!(gb.get_memory_at(0x9910), 0xFF);
        }
    }
}
