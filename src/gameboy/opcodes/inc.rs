use super::super::{write_flag, Flags};
use super::{Argument, OpCode, ReadWriteRegister};

impl OpCode {
    pub fn run_inc<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        _memory: &mut Vec<u8>,
    ) -> u32 {
        let mut cycles = 0;
        if let Argument::Register8Constant(reg) = self.args[0] {
            let reg_value = cpu.read_8_bits(reg);

            let (new_val, overflow) = reg_value.overflowing_add(1);

            if overflow {
                write_flag::<T>(cpu, Flags::Z, true);
            }

            write_flag::<T>(cpu, Flags::N, false);

            if new_val == (0x0F + 1) {
                write_flag::<T>(cpu, Flags::H, true);
            }

            cpu.write_8_bits(reg, new_val);

            cycles += 4;
        }
        cycles
    }
}
