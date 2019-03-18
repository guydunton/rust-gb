use super::{read_flag, write_flag, Flags, OpCode, ReadWriteRegister, RegisterLabel8};

impl OpCode {
    pub fn run_rla<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        _memory: &mut Vec<u8>,
    ) -> u32 {
        let mut cycles = 0;
        let mask = 0b1000_0000;
        let reg_contents = cpu.read_8_bits(RegisterLabel8::A);
        let eighth_bit = (reg_contents & mask) >> 7;

        let carry_flag = read_flag::<T>(cpu, Flags::C);

        // Create the new register value
        let new_register = (reg_contents << 1) | (carry_flag as u8);

        // Set the carry flag
        write_flag::<T>(cpu, Flags::C, eighth_bit == 1);

        // Write away the flag
        cpu.write_8_bits(RegisterLabel8::A, new_register);

        // Unset the H & N flags
        write_flag::<T>(cpu, Flags::H, false);
        write_flag::<T>(cpu, Flags::N, false);

        // Check the result in the C register for 0 to set the zero flag
        if new_register == 0 {
            write_flag::<T>(cpu, Flags::Z, true);
        }

        cycles += 4;
        cycles
    }
}
