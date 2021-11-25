#[cfg(test)]
mod load8_test {

    use crate::gameboy::cpu::CPU;
    use crate::gameboy::memory_adapter::MemoryAdapter;
    use crate::gameboy::opcodes::{Argument, Category, Decoder};
    use crate::gameboy::{Gameboy, OpCode};
    use crate::gameboy::{RegisterLabel16, RegisterLabel8};

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

        // LD C d8
        ld8_test(0x0E, RegisterLabel8::C);

        // LD D d8
        ld8_test(0x16, RegisterLabel8::D);

        // LD L d8
        ld8_test(0x2E, RegisterLabel8::L);

        // LD A d8
        ld8_test(0x3E, RegisterLabel8::A);

        ld8_test(0x1E, RegisterLabel8::E);

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

    #[test]
    fn ld8_hl_plus_a() {
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

    #[test]
    fn ldh_a8_a() {
        // LDH (a8) A
        let mut gb = Gameboy::new(vec![0xE0, 0x01]);
        gb.set_register_8(RegisterLabel8::A, 0x02);

        let cycles = gb.step_once();

        assert_eq!(gb.get_memory_at(0xFF01) as usize, 0x02);
        assert_eq!(cycles, 12);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);
    }

    #[test]
    fn ldh_a_a8() {
        let mut gb = Gameboy::new(vec![0xF0, 0x02]);
        gb.set_memory_at(0xFF02, 0x34);

        let cycles = gb.step_once();

        println!("A register: {:?}", gb.get_register_8(RegisterLabel8::A));

        assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x34);
        assert_eq!(cycles, 12);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);
    }

    #[test]
    fn ld8_into_address_address() {
        let mut gb = Gameboy::new(vec![0xEA, 0x10, 0x99]); // LD8 ($9910), A

        gb.set_register_8(RegisterLabel8::A, 0xFF);

        let cycles = gb.step_once();

        assert_eq!(cycles, 16);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x03);
        assert_eq!(gb.get_memory_at(0x9910), 0xFF);
    }

    #[test]
    fn ld8_hl_d8() {
        let mut gb = Gameboy::new(vec![0x36, 0x12]);
        gb.set_register_16(RegisterLabel16::HL, 0xFF15);
        let cycles = gb.step_once();

        assert_eq!(cycles, 12);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x02);
        assert_eq!(gb.get_memory_at(0xFF15), 0x12);
    }

    #[test]
    fn ld8_a_hlplus() {
        let mut gb = Gameboy::new(vec![0x2A]);
        gb.set_register_16(RegisterLabel16::HL, 0xFF00);
        gb.set_memory_at(0xFF00, 0x12);
        let cycles = gb.step_once();

        assert_eq!(cycles, 8);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
        assert_eq!(gb.get_register_16(RegisterLabel16::HL), 0xFF01);
        assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x12);
    }

    #[test]
    fn ld8_can_move_between_registers() {
        let mut cpu = CPU::new();
        let mut memory = vec![0x0; 0xFFFF];

        let opcode = OpCode::new(
            Category::LD8,
            [
                Argument::Register8Constant(RegisterLabel8::A),
                Argument::Register8Constant(RegisterLabel8::B),
            ],
        );

        cpu.write_8_bits(RegisterLabel8::B, 0x01);

        let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        // Check the result
        assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x01);

        // Check the size
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);
        assert_eq!(cycles, 4);

        // Check another set of registers just in case
        let opcode = OpCode::new(
            Category::LD8,
            [
                Argument::Register8Constant(RegisterLabel8::F),
                Argument::Register8Constant(RegisterLabel8::D),
            ],
        );

        cpu.write_8_bits(RegisterLabel8::D, 0xFF);
        let _ = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        assert_eq!(cpu.read_8_bits(RegisterLabel8::F), 0xFF);
    }

    #[test]
    fn ld8_can_move_to_hl_offset() {
        // Create instruction

        // Run

        // Check the result

        // Check the size
    }

    #[test]
    fn l8_can_move_from_hl_offset() {
        // Create instruction
        let opcode = OpCode::new(
            Category::LD8,
            [
                Argument::Register8Constant(RegisterLabel8::A),
                Argument::RegisterIndirect(RegisterLabel16::HL),
            ],
        );

        let mut cpu = CPU::new();
        let mut memory = vec![0x0; 0xFFFF];

        // Set the memory & HL register
        memory[0xFF00] = 0x01;
        cpu.write_16_bits(RegisterLabel16::HL, 0xFF00);

        let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        // Check the result
        assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x01);

        // Check the size
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn ld8_from_r8_into_hl() {
        let opcode = OpCode::new(
            Category::LD8,
            [
                Argument::RegisterIndirect(RegisterLabel16::HL),
                Argument::Register8Constant(RegisterLabel8::B),
            ],
        );

        let mut cpu = CPU::new();
        let mut memory = vec![0x0; 0xFFFF];

        cpu.write_16_bits(RegisterLabel16::HL, 0xFF00);
        cpu.write_8_bits(RegisterLabel8::B, 0x12);

        let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        assert_eq!(cycles, 8);
        assert_eq!(memory[0xFF00], 0x12);
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x1);
    }

    #[test]
    fn ld_a_hl_decrement() {
        let opcode = OpCode::new(
            Category::LD8,
            [
                Argument::Register8Constant(RegisterLabel8::A),
                Argument::RegisterIndirectDec(RegisterLabel16::HL),
            ],
        );

        let mut cpu = CPU::new();
        let mut memory = vec![0x0; 0xFFFF];

        cpu.write_16_bits(RegisterLabel16::HL, 0xFF00);
        memory[0xFF00] = 0x12;

        let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0x12);
        assert_eq!(cpu.read_16_bits(RegisterLabel16::HL), 0xFEFF);
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0x01);
        assert_eq!(cycles, 8);
    }

    fn r8_r8_opcode(dest_r8: RegisterLabel8, src_r8: RegisterLabel8) -> OpCode {
        OpCode::new(
            Category::LD8,
            [
                Argument::Register8Constant(dest_r8),
                Argument::Register8Constant(src_r8),
            ],
        )
    }

    fn r8_hl_opcode(dest: RegisterLabel8) -> OpCode {
        OpCode::new(
            Category::LD8,
            [
                Argument::Register8Constant(dest),
                Argument::RegisterIndirect(RegisterLabel16::HL),
            ],
        )
    }

    fn hl_r8_opcode(src: RegisterLabel8) -> OpCode {
        OpCode::new(
            Category::LD8,
            [
                Argument::RegisterIndirect(RegisterLabel16::HL),
                Argument::Register8Constant(src),
            ],
        )
    }

    #[test]
    fn ld_h_d8() {
        let opcode = OpCode::new(
            Category::LD8,
            [
                Argument::Register8Constant(RegisterLabel8::A),
                Argument::SmallValue(0x12),
            ],
        );

        let mut cpu = CPU::new();
        let mut memory = vec![0x0; 0xFFFF];

        let cycles = opcode.run(&mut cpu, MemoryAdapter::new(&mut memory));

        assert_eq!(cycles, 8);
        assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 2);
    }

    #[test]
    fn ld8_is_decoded_correctly() {
        let decode = |memory| Decoder::decode_instruction(0x00, memory).unwrap();

        const A: RegisterLabel8 = RegisterLabel8::A;
        const B: RegisterLabel8 = RegisterLabel8::B;
        const C: RegisterLabel8 = RegisterLabel8::C;
        const D: RegisterLabel8 = RegisterLabel8::D;
        const E: RegisterLabel8 = RegisterLabel8::E;
        const H: RegisterLabel8 = RegisterLabel8::H;
        const L: RegisterLabel8 = RegisterLabel8::L;

        // All r8 -> r8 instructions
        assert_eq!(decode(&[0x40]), r8_r8_opcode(B, B));
        assert_eq!(decode(&[0x41]), r8_r8_opcode(B, C));
        assert_eq!(decode(&[0x42]), r8_r8_opcode(B, D));
        assert_eq!(decode(&[0x43]), r8_r8_opcode(B, E));
        assert_eq!(decode(&[0x44]), r8_r8_opcode(B, H));
        assert_eq!(decode(&[0x45]), r8_r8_opcode(B, L));
        assert_eq!(decode(&[0x47]), r8_r8_opcode(B, A));
        assert_eq!(decode(&[0x48]), r8_r8_opcode(C, B));
        assert_eq!(decode(&[0x49]), r8_r8_opcode(C, C));
        assert_eq!(decode(&[0x4A]), r8_r8_opcode(C, D));
        assert_eq!(decode(&[0x4B]), r8_r8_opcode(C, E));
        assert_eq!(decode(&[0x4C]), r8_r8_opcode(C, H));
        assert_eq!(decode(&[0x4D]), r8_r8_opcode(C, L));

        assert_eq!(decode(&[0x50]), r8_r8_opcode(D, B));
        assert_eq!(decode(&[0x51]), r8_r8_opcode(D, C));
        assert_eq!(decode(&[0x52]), r8_r8_opcode(D, D));
        assert_eq!(decode(&[0x53]), r8_r8_opcode(D, E));
        assert_eq!(decode(&[0x54]), r8_r8_opcode(D, H));
        assert_eq!(decode(&[0x55]), r8_r8_opcode(D, L));
        assert_eq!(decode(&[0x57]), r8_r8_opcode(D, A));
        assert_eq!(decode(&[0x58]), r8_r8_opcode(E, B));
        assert_eq!(decode(&[0x59]), r8_r8_opcode(E, C));
        assert_eq!(decode(&[0x5A]), r8_r8_opcode(E, D));
        assert_eq!(decode(&[0x5B]), r8_r8_opcode(E, E));
        assert_eq!(decode(&[0x5C]), r8_r8_opcode(E, H));
        assert_eq!(decode(&[0x5D]), r8_r8_opcode(E, L));
        assert_eq!(decode(&[0x5F]), r8_r8_opcode(E, A));

        assert_eq!(decode(&[0x60]), r8_r8_opcode(H, B));
        assert_eq!(decode(&[0x61]), r8_r8_opcode(H, C));
        assert_eq!(decode(&[0x62]), r8_r8_opcode(H, D));
        assert_eq!(decode(&[0x63]), r8_r8_opcode(H, E));
        assert_eq!(decode(&[0x64]), r8_r8_opcode(H, H));
        assert_eq!(decode(&[0x65]), r8_r8_opcode(H, L));

        assert_eq!(decode(&[0x67]), r8_r8_opcode(H, A));
        assert_eq!(decode(&[0x68]), r8_r8_opcode(L, B));
        assert_eq!(decode(&[0x69]), r8_r8_opcode(L, C));
        assert_eq!(decode(&[0x6A]), r8_r8_opcode(L, D));
        assert_eq!(decode(&[0x6B]), r8_r8_opcode(L, E));
        assert_eq!(decode(&[0x6C]), r8_r8_opcode(L, H));
        assert_eq!(decode(&[0x6D]), r8_r8_opcode(L, L));
        assert_eq!(decode(&[0x6F]), r8_r8_opcode(L, A));

        assert_eq!(decode(&[0x78]), r8_r8_opcode(A, B));
        assert_eq!(decode(&[0x79]), r8_r8_opcode(A, C));
        assert_eq!(decode(&[0x7A]), r8_r8_opcode(A, D));
        assert_eq!(decode(&[0x7B]), r8_r8_opcode(A, E));
        assert_eq!(decode(&[0x7C]), r8_r8_opcode(A, H));
        assert_eq!(decode(&[0x7D]), r8_r8_opcode(A, L));
        assert_eq!(decode(&[0x7F]), r8_r8_opcode(A, A));

        // All r8 <- (HL) instructions
        assert_eq!(decode(&[0x46]), r8_hl_opcode(B));
        assert_eq!(decode(&[0x4E]), r8_hl_opcode(C));
        assert_eq!(decode(&[0x56]), r8_hl_opcode(D));
        assert_eq!(decode(&[0x5E]), r8_hl_opcode(E));
        assert_eq!(decode(&[0x66]), r8_hl_opcode(H));
        assert_eq!(decode(&[0x6E]), r8_hl_opcode(L));
        assert_eq!(decode(&[0x7E]), r8_hl_opcode(A));

        // All (HL) <- r8 instructions
        assert_eq!(decode(&[0x70]), hl_r8_opcode(B));
        assert_eq!(decode(&[0x71]), hl_r8_opcode(C));
        assert_eq!(decode(&[0x72]), hl_r8_opcode(D));
        assert_eq!(decode(&[0x73]), hl_r8_opcode(E));
        assert_eq!(decode(&[0x74]), hl_r8_opcode(H));
        assert_eq!(decode(&[0x75]), hl_r8_opcode(L));
        assert_eq!(decode(&[0x77]), hl_r8_opcode(A));

        assert_eq!(
            decode(&[0x02]),
            OpCode::new(
                Category::LD8,
                [
                    Argument::RegisterIndirect(RegisterLabel16::BC),
                    Argument::Register8Constant(RegisterLabel8::A)
                ]
            )
        );
        assert_eq!(
            decode(&[0x12]),
            OpCode::new(
                Category::LD8,
                [
                    Argument::RegisterIndirect(RegisterLabel16::DE),
                    Argument::Register8Constant(RegisterLabel8::A)
                ]
            )
        );

        assert_eq!(
            decode(&[0x0A]),
            OpCode::new(
                Category::LD8,
                [
                    Argument::Register8Constant(RegisterLabel8::A),
                    Argument::RegisterIndirect(RegisterLabel16::BC)
                ]
            )
        );

        assert_eq!(
            decode(&[0x3A]),
            OpCode::new(
                Category::LD8,
                [
                    Argument::Register8Constant(RegisterLabel8::A),
                    Argument::RegisterIndirectDec(RegisterLabel16::HL)
                ]
            )
        );

        assert_eq!(
            decode(&[0x26, 0x12]),
            OpCode::new(
                Category::LD8,
                [
                    Argument::Register8Constant(RegisterLabel8::H),
                    Argument::SmallValue(0x12)
                ]
            )
        );
    }
}
