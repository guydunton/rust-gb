#[cfg(test)]
mod opcode_printer_tests {

    use crate::gameboy::register::RegisterLabel16;
    use crate::Gameboy;
    use rust_catch::tests;

    tests! {
        test("Can print an instruction as a string") {
            let gb = Gameboy::new(vec![0x00]);
            let current_instruction = gb.get_current_instruction();

            assert_eq!(current_instruction, "NOP".to_owned());
        }

        test("Can get instruction plus offset") {
            let gb = Gameboy::new(vec![0x00, 0x0C]);
            let next_instruction = gb.get_instruction_offset(1).unwrap();

            assert_eq!(next_instruction, "INC C".to_owned());
        }

        test("Can get the instruction minus an offset") {
            let mut gb = Gameboy::new(vec![0x00, 0x0C]);
            gb.set_register_16(RegisterLabel16::ProgramCounter, 0x01);
            let prev_instruction = gb.get_instruction_offset(-1).unwrap();
            assert_eq!(prev_instruction, "NOP".to_owned());
        }
    }

    #[test]
    fn will_fail_getting_an_instruction_out_of_range() {
        let mut gb = Gameboy::new(vec![0x00]);
        gb.set_register_16(RegisterLabel16::ProgramCounter, 10);
        let next_instruction = gb.get_instruction_offset((u16::max_value() - 10) as i32);

        assert!(next_instruction.is_err());
    }
}
