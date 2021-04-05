#[cfg(test)]
mod pop_test {
    use crate::gameboy::Gameboy;
    use crate::gameboy::RegisterLabel16;

    #[test]
    fn pop_instruction_moves_the_stack_pointer() {
        // POP BC. The test has space then the
        let mut gb = Gameboy::new(vec![0xC1, 0x00, 0x01, 0x23]);

        // setup the stack pointer
        gb.set_register_16(RegisterLabel16::StackPointer, 0x02);

        let cycles = gb.step_once();

        // section("The stack shrinks upwards") {
        // The stack grows downwards so the pop instruction
        // moves the stack pointer upwards
        assert_eq!(gb.get_register_16(RegisterLabel16::StackPointer), 0x4);

        // section("The instruction takes 12 cycles and 1 byte") {
        assert_eq!(cycles, 12);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1);

        // section("The contents of the stack are put in BC") {
        let bc = gb.get_register_16(RegisterLabel16::BC);
        assert_eq!(bc, 0x2301);
    }
}
