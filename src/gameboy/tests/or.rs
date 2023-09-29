use crate::gameboy::{Flags, Gameboy, RegisterLabel16, RegisterLabel8};

#[test]
fn or_r8_test() {
    let opcodes = vec![
        (0xB0, RegisterLabel8::B),
        (0xB1, RegisterLabel8::C),
        (0xB2, RegisterLabel8::D),
        (0xB3, RegisterLabel8::E),
        (0xB4, RegisterLabel8::H),
        (0xB5, RegisterLabel8::L),
    ];

    for (opcode, register) in &opcodes {
        let mut gb = Gameboy::new(vec![*opcode]);

        // Set the register
        gb.set_register_8(*register, 0x1);
        gb.set_register_8(RegisterLabel8::A, 0x0);

        let cycles = gb.step_once().unwrap();

        // Check the flags are zero
        assert_eq!(gb.get_register_8(RegisterLabel8::F), 0);

        // Check the program counter & cycles
        assert_eq!(gb.get_register_16(RegisterLabel16::ProgramCounter), 0x1);
        assert_eq!(cycles, 4);
    }

    // Check that the zero flag is set if the result is zero
    for (opcode, register) in &opcodes {
        let mut gb = Gameboy::new(vec![*opcode]);

        gb.set_register_8(*register, 0x0);
        gb.set_register_8(RegisterLabel8::A, 0x0);

        let _ = gb.step_once();

        assert_eq!(gb.get_flag(Flags::Z), true);
    }
}

#[test]
fn writing_to_a_works() {
    let mut gb = Gameboy::new(vec![0xB7, 0xB7]);

    gb.set_register_8(RegisterLabel8::A, 0);

    gb.step_once();

    assert_eq!(gb.get_flag(Flags::Z), true);
    assert_eq!(gb.get_register_8(RegisterLabel8::A), 0);

    // Check the reverse
    gb.set_register_8(RegisterLabel8::A, 1);
    gb.step_once();

    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_register_8(RegisterLabel8::A), 1);
}
