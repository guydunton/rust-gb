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
            // LD C A
            let mut gb = Gameboy::new(vec![0xE2]);
            gb.set_register_8(RegisterLabel8::C, 0x01);
            gb.set_register_8(RegisterLabel8::A, 0x02);

            let cycles = gb.step_once();

            assert_eq!(gb.get_memory_at(0xFF01), 0x02);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
            assert_eq!(cycles, 8);
        }

        {
            // LD (HL) A
            let mut gb = Gameboy::new(vec![0x77]);
            gb.set_register_16(RegisterLabel16::HL, 0x0005);
            gb.set_register_8(RegisterLabel8::A, 0x01);
            let cycles = gb.step_once();

            assert_eq!(gb.get_memory_at(0x0005), 0x01);
            assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
            assert_eq!(cycles, 8);
        }

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
    fn generic_ld8_r8_r8_test() {
        let instructions = vec![
            (0x7B, RegisterLabel8::A, RegisterLabel8::E),
            (0x67, RegisterLabel8::H, RegisterLabel8::A),
            (0x57, RegisterLabel8::D, RegisterLabel8::A),
            (0x7C, RegisterLabel8::A, RegisterLabel8::H),
        ];

        for &(code, dest, src) in instructions.iter() {
            let mut gb = Gameboy::new(vec![code]);

            gb.set_register_8(src, 0x04);
            gb.set_register_8(dest, 0x01);

            let _ = gb.step_once();

            assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x04);
        }
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
    fn ld8_a_l() {
        let mut gb = Gameboy::new(vec![0x7D]);
        gb.set_register_8(RegisterLabel8::A, 0x0);
        gb.set_register_8(RegisterLabel8::L, 0x18);

        let cycles = gb.step_once();

        assert_eq!(cycles, 4);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
        assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x18);
    }

    #[test]
    fn ld8_a_b() {
        let mut gb = Gameboy::new(vec![0x78]);
        gb.set_register_8(RegisterLabel8::A, 0x0);
        gb.set_register_8(RegisterLabel8::B, 0x18);
        let cycles = gb.step_once();

        assert_eq!(cycles, 4);
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x01);
        assert_eq!(gb.get_register_8(RegisterLabel8::A), 0x18);
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

        // Run

        // Check the result

        // Check the size
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

    #[test]
    fn ld8_is_decoded_correctly() {
        let decode = |memory| Decoder::decode_instruction(0x00, memory).unwrap();

        const A: RegisterLabel8 = RegisterLabel8::A;
        const B: RegisterLabel8 = RegisterLabel8::B;
        const C: RegisterLabel8 = RegisterLabel8::C;
        const D: RegisterLabel8 = RegisterLabel8::D;
        const E: RegisterLabel8 = RegisterLabel8::E;
        const F: RegisterLabel8 = RegisterLabel8::F;
        const H: RegisterLabel8 = RegisterLabel8::H;
        const L: RegisterLabel8 = RegisterLabel8::L;

        // All r8 -> r8 instructions
        assert_eq!(decode(&[0x40]), r8_r8_opcode(B, B));
        assert_eq!(decode(&[0x41]), r8_r8_opcode(B, C));
        assert_eq!(decode(&[0x42]), r8_r8_opcode(B, D));
        assert_eq!(decode(&[0x43]), r8_r8_opcode(B, E));
        assert_eq!(decode(&[0x44]), r8_r8_opcode(B, H));
        assert_eq!(decode(&[0x45]), r8_r8_opcode(B, L));

        // All r8 -> (HL) instructions

        //
    }
}
