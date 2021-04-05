#[cfg(test)]
mod opcode_printer_tests {

    use crate::gameboy::register::RegisterLabel16;
    use crate::Gameboy;

    #[test]
    fn can_print_an_instruction_as_a_string() {
        let gb = Gameboy::new(vec![0x00]);
        let current_instruction = gb.get_current_instruction();

        assert_eq!(current_instruction.unwrap(), "NOP".to_owned());
    }

    #[test]
    fn can_get_instruction_plus_offset() {
        let gb = Gameboy::new(vec![0x00, 0x0C]);
        let (next_instruction, address) = gb.get_opcode_with_offset(1).unwrap();

        assert_eq!(next_instruction, "INC C".to_owned());
        assert_eq!(address, 0x01);
    }

    #[test]
    fn get_a_second_instruction_correctly() {
        let gb = Gameboy::new(vec![0x31, 0xFE, 0xFF, 0x00]);
        let (next_instruction, address) = gb.get_opcode_with_offset(1).unwrap();

        assert_eq!(next_instruction, "NOP".to_owned());
        assert_eq!(address, 0x03);
    }

    #[test]
    fn will_fail_getting_an_instruction_out_of_range() {
        let mut gb = Gameboy::new(vec![0x00]);
        gb.set_register_16(RegisterLabel16::ProgramCounter, 10);
        let next_instruction = gb.get_opcode_with_offset(u16::max_value() - 10);

        assert!(next_instruction.is_err());
    }
}
