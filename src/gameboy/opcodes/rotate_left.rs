use super::rotate_method::shift_reg_and_flag;
use super::{read_flag, write_flag, Argument, Flags, OpCode, ReadWriteRegister};

impl OpCode {
    pub fn run_rl<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        _memory: &mut Vec<u8>,
    ) -> u32 {
        let mut cycles = 0;
        if let Argument::Register8Constant(reg) = self.args[0] {
            let reg_contents = cpu.read_8_bits(reg);
            let carry_flag = read_flag::<T>(cpu, Flags::C);

            let (new_register, new_carry) = shift_reg_and_flag(reg_contents, carry_flag);

            // Set the carry flag
            write_flag::<T>(cpu, Flags::C, new_carry);

            // Unset the H & N flags
            write_flag::<T>(cpu, Flags::H, false);
            write_flag::<T>(cpu, Flags::N, false);

            // Write away the flag
            cpu.write_8_bits(reg, new_register);

            // Check the result in the C register for 0 to set the zero flag
            if new_register == 0 {
                write_flag::<T>(cpu, Flags::Z, true);
            }

            cycles += 8;
        }
        cycles
    }
}
