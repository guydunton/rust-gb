use super::OpCode;
use super::ReadWriteRegister;
use super::RegisterLabel16;

impl OpCode {
    pub fn run_ret<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        memory: &mut Vec<u8>,
    ) -> u32 {
        let stack_pointer = cpu.read_16_bits(RegisterLabel16::StackPointer);

        // Get the top 2 bytes of the stack
        let lower_byte = memory[stack_pointer as usize];
        let higher_byte = memory[stack_pointer as usize + 1];

        // Move the stack pointer
        cpu.write_16_bits(RegisterLabel16::StackPointer, stack_pointer + 2);

        // Set the program counter to the value from the stack
        let new_program_counter = u16::from_be_bytes([higher_byte, lower_byte]);

        cpu.write_16_bits(RegisterLabel16::ProgramCounter, new_program_counter);
        16
    }
}
