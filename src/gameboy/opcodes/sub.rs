use super::super::{write_flag, Flags, OpCode};
use super::{Argument, ReadWriteRegister, RegisterLabel8};

impl OpCode {
    pub fn run_sub<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        _memory: &mut Vec<u8>,
    ) -> u32 {
        let cycles = 4;

        // Clear all the flags
        cpu.write_8_bits(RegisterLabel8::F, 0);

        // Get the register argument
        if let Argument::Register8Constant(reg) = self.args[0] {
            // Read the registers
            let reg_value = cpu.read_8_bits(reg);
            let a_reg_value = cpu.read_8_bits(RegisterLabel8::A);

            // Subtract one from the other
            let result = a_reg_value - reg_value; // This is underflowing

            // Write away the A flag
            cpu.write_8_bits(RegisterLabel8::A, result);

            if reg_value == a_reg_value {
                write_flag::<T>(cpu, Flags::Z, true);
            }

            if reg_value > a_reg_value {
                write_flag::<T>(cpu, Flags::C, true);
            }
        }

        // Set the N flag
        write_flag::<T>(cpu, Flags::N, true);

        cycles
    }
}
