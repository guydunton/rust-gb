use super::register::{RegisterLabel16, RegisterLabel8, RegisterPair};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
pub struct CPU {
    registers: Vec<RegisterPair>,
}

impl CPU {
    pub fn new() -> CPU {
        let registers = vec![
            RegisterPair::new(RegisterLabel16::ProgramCounter),
            RegisterPair::new(RegisterLabel16::StackPointer),
            RegisterPair::new_with_8_bit_registers(
                RegisterLabel16::AF,
                [RegisterLabel8::A, RegisterLabel8::F],
            ),
            RegisterPair::new_with_8_bit_registers(
                RegisterLabel16::BC,
                [RegisterLabel8::B, RegisterLabel8::C],
            ),
            RegisterPair::new_with_8_bit_registers(
                RegisterLabel16::DE,
                [RegisterLabel8::D, RegisterLabel8::E],
            ),
            RegisterPair::new_with_8_bit_registers(
                RegisterLabel16::HL,
                [RegisterLabel8::H, RegisterLabel8::L],
            ),
        ];

        CPU { registers }
    }

    pub fn write_16_bits(&mut self, label: RegisterLabel16, value: u16) {
        self.registers
            .iter_mut()
            .find(|register| register.contains_16_bit_register(label))
            .expect("Couldn't find specified 16 bit register")
            .perform_16_bit_write(value);
    }

    pub fn write_8_bits(&mut self, label: RegisterLabel8, value: u8) {
        self.registers
            .iter_mut()
            .find(|register| register.contains_8_bit_register(label))
            .and_then(|register| register.perform_8_bit_write(label, value))
            .expect("Couldn't find specified 8 bit register");
    }

    pub fn read_16_bits(&self, label: RegisterLabel16) -> u16 {
        self.registers
            .iter()
            .find(|register| register.contains_16_bit_register(label))
            .expect("Couldn't find specified 16 bit register")
            .perform_16_bit_read()
    }

    pub fn read_8_bits(&self, label: RegisterLabel8) -> u8 {
        self.registers
            .iter()
            .find(|register| register.contains_8_bit_register(label))
            .and_then(|register| register.perform_8_bit_read(label))
            .expect("Couldn't find the specified 8 bit register")
    }
}

//-------------------------------------------------------
// Tests
//-------------------------------------------------------

#[test]
fn created_cpu_is_zero() {
    let cpu = CPU::new();
    assert_eq!(cpu.read_16_bits(RegisterLabel16::AF), 0);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::BC), 0);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::DE), 0);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::HL), 0);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::ProgramCounter), 0);
    assert_eq!(cpu.read_16_bits(RegisterLabel16::StackPointer), 0);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::A), 0);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::F), 0);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::B), 0);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::C), 0);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::D), 0);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::E), 0);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::H), 0);
    assert_eq!(cpu.read_8_bits(RegisterLabel8::L), 0);
}

#[allow(unused)]
fn write_8_assert(cpu: &mut CPU, label: RegisterLabel8, val: u8) {
    static REGISTERS: [RegisterLabel8; 8] = [
        RegisterLabel8::A,
        RegisterLabel8::F,
        RegisterLabel8::B,
        RegisterLabel8::C,
        RegisterLabel8::D,
        RegisterLabel8::E,
        RegisterLabel8::H,
        RegisterLabel8::L,
    ];

    let cpu_copy = cpu.clone();
    cpu.write_8_bits(label, val);
    assert_eq!(cpu.read_8_bits(label), val);

    // Make sure the other registers have not been changed by this
    for r in REGISTERS.iter() {
        if *r != label {
            assert_eq!(cpu_copy.read_8_bits(*r), cpu.read_8_bits(*r));
        }
    }
}

#[allow(unused)]
fn write_16_assert(cpu: &mut CPU, label: RegisterLabel16, val: u16) {
    cpu.write_16_bits(label, val);
    let result = cpu.read_16_bits(label);
    assert_eq!(result, val);
}

#[test]
fn can_write_then_read() {
    let mut cpu = CPU::new();
    write_8_assert(&mut cpu, RegisterLabel8::A, 0x01);
    write_8_assert(&mut cpu, RegisterLabel8::F, 0x02);
    write_8_assert(&mut cpu, RegisterLabel8::B, 0x03);
    write_8_assert(&mut cpu, RegisterLabel8::C, 0x04);
    write_8_assert(&mut cpu, RegisterLabel8::D, 0x05);
    write_8_assert(&mut cpu, RegisterLabel8::E, 0x06);
    write_8_assert(&mut cpu, RegisterLabel8::H, 0x07);
    write_8_assert(&mut cpu, RegisterLabel8::L, 0x08);

    write_16_assert(&mut cpu, RegisterLabel16::AF, 0x1234);
    write_16_assert(&mut cpu, RegisterLabel16::BC, 0x2345);
    write_16_assert(&mut cpu, RegisterLabel16::DE, 0x3456);
    write_16_assert(&mut cpu, RegisterLabel16::HL, 0x4567);
    write_16_assert(&mut cpu, RegisterLabel16::StackPointer, 0x5678);
    write_16_assert(&mut cpu, RegisterLabel16::ProgramCounter, 0x6789);
}

#[allow(unused)]
fn write_2_registers_read_16(labels_8: [RegisterLabel8; 2], labels_16: RegisterLabel16) {
    let mut cpu = CPU::new();
    cpu.write_8_bits(labels_8[0], 0x01);
    cpu.write_8_bits(labels_8[1], 0x23);
    assert_eq!(cpu.read_16_bits(labels_16), 0x0123);
}

#[test]
fn can_write_8_read_16() {
    write_2_registers_read_16([RegisterLabel8::A, RegisterLabel8::F], RegisterLabel16::AF);
    write_2_registers_read_16([RegisterLabel8::B, RegisterLabel8::C], RegisterLabel16::BC);
    write_2_registers_read_16([RegisterLabel8::D, RegisterLabel8::E], RegisterLabel16::DE);
    write_2_registers_read_16([RegisterLabel8::H, RegisterLabel8::L], RegisterLabel16::HL);
}
