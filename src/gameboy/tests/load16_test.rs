#[cfg(test)]
mod load16_test {
    use crate::gameboy::cpu::CPU;
    use crate::gameboy::memory_adapter::MemoryAdapter;
    use crate::gameboy::opcodes::{Argument, Category, Decoder};
    use crate::gameboy::OpCode;
    use crate::gameboy::RegisterLabel16;

    #[test]
    fn fixed_value_ld16_test() {
        let opcode = OpCode::new(
            Category::LD16,
            [
                Argument::Register16Constant(RegisterLabel16::StackPointer),
                Argument::LargeValue(0x1234),
            ],
        );

        let mut memory = vec![0; 0xFFFF];
        let mut cpu = CPU::new();

        let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        assert_eq!(cpu.read_16_bits(RegisterLabel16::StackPointer), 0x1234);
        assert_eq!(cycles, 12);

        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x0003);
    }

    #[test]
    fn address_ld16() {
        let opcode = OpCode::new(
            Category::LD16,
            [
                Argument::AddressIndirect(0x0003),
                Argument::Register16Constant(RegisterLabel16::StackPointer),
            ],
        );

        let mut memory = vec![0; 0xFFFF];
        let mut cpu = CPU::new();

        cpu.write_16_bits(RegisterLabel16::StackPointer, 0x1234);

        let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        assert_eq!(memory[0x03], 0x34);
        assert_eq!(memory[0x04], 0x12);

        assert_eq!(cycles, 20);
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x03);
    }

    #[test]
    fn register_ld16() {
        let opcode = OpCode::new(
            Category::LD16,
            [
                Argument::Register16Constant(RegisterLabel16::StackPointer),
                Argument::Register16Constant(RegisterLabel16::HL),
            ],
        );

        let mut memory = vec![0; 0xFFFF];
        let mut cpu = CPU::new();

        cpu.write_16_bits(RegisterLabel16::HL, 0x1234);

        let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        assert_eq!(cpu.read_16_bits(RegisterLabel16::StackPointer), 0x1234);
        assert_eq!(cycles, 8);
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);
    }

    fn ld_opcode(dest: RegisterLabel16, val: u16) -> OpCode {
        OpCode::new(
            Category::LD16,
            [
                Argument::Register16Constant(dest),
                Argument::LargeValue(val),
            ],
        )
    }

    #[test]
    fn ld_decoding_test() {
        // 0x01: LD BC,d16
        let opcode = Decoder::decode_instruction(0x0000, &vec![0x01, 0x34, 0x12]).unwrap();
        assert_eq!(opcode, ld_opcode(RegisterLabel16::BC, 0x1234));

        // 0x11: LD DE,d16
        let opcode = Decoder::decode_instruction(0x0000, &vec![0x11, 0x56, 0x34]).unwrap();
        assert_eq!(opcode, ld_opcode(RegisterLabel16::DE, 0x3456));

        // 0x21: LD HL,d16
        let opcode = Decoder::decode_instruction(0x0000, &vec![0x21, 0x89, 0x67]).unwrap();
        assert_eq!(opcode, ld_opcode(RegisterLabel16::HL, 0x6789));

        // 0x31: LD SP,d16
        let opcode = Decoder::decode_instruction(0x0000, &vec![0x31, 0xFE, 0xFF]).unwrap();
        assert_eq!(opcode, ld_opcode(RegisterLabel16::StackPointer, 0xFFFE));

        // 0x08: LD (a16),SP
        let opcode = Decoder::decode_instruction(0x0000, &vec![0x08, 0x34, 0x12]).unwrap();
        assert_eq!(
            opcode,
            OpCode::new(
                Category::LD16,
                [
                    Argument::AddressIndirect(0x1234),
                    Argument::Register16Constant(RegisterLabel16::StackPointer)
                ]
            )
        );

        // 0xF9: LD SP,HL
        let opcode = Decoder::decode_instruction(0x0000, &vec![0xF9]).unwrap();
        assert_eq!(
            opcode,
            OpCode::new(
                Category::LD16,
                [
                    Argument::Register16Constant(RegisterLabel16::StackPointer),
                    Argument::Register16Constant(RegisterLabel16::HL)
                ]
            )
        );
    }
}
