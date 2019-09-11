mod cpu;
pub mod flags_register;
mod read_write_register;
pub mod register;

pub mod opcodes;
pub mod screen;

mod tests;

use self::cpu::CPU;
use self::flags_register::{read_flag, write_flag, Flags};
use self::opcodes::OpCode;
use self::read_write_register::ReadWriteRegister;
use self::register::{RegisterLabel16, RegisterLabel8};

pub struct Gameboy {
    cpu: CPU,
    memory: Vec<u8>,
}

impl Gameboy {
    pub fn new_with_bootloader() -> Gameboy {
        let bootloader = vec![
            0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26,
            0xFF, 0x0E, // 0x0010
            0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E,
            0xFC, 0xE0, // 0x0020
            0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00,
            0x13, 0x7B, // 0x0030
            0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05,
            0x20, 0xF9, // 0x0040
            0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32,
            0x0D, 0x20, 0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E,
            0x91, 0xE0, 0x40, 0x04, 0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA,
            0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2, 0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62,
            0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06, 0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2,
            0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20, 0x4F, 0x16, 0x20, 0x18,
            0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17, 0x05, 0x20,
            0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
            0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89,
            0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63,
            0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42,
            0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C, 0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13,
            0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20, 0xF5, 0x06, 0x19, 0x78, 0x86, 0x23,
            0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
        ];

        Gameboy::new(bootloader)
    }

    #[allow(dead_code)]
    pub fn new(data: Vec<u8>) -> Gameboy {
        let mut memory = vec![0; 0xFFFF];
        memory[..data.len()].clone_from_slice(&data[..]);

        Gameboy {
            cpu: CPU::new(),
            memory,
        }
    }

    #[allow(dead_code)]
    pub fn tick(&mut self, dt: f64) {
        let cycles_to_use = (dt * 1000000f64) as u32;
        let mut total_cycles_used = 0;

        loop {
            let opcode = self.current_opcode();

            let cycles_used = opcode.run::<CPU>(&mut self.cpu, &mut self.memory);

            total_cycles_used += cycles_used;
            if total_cycles_used > cycles_to_use {
                break;
            }
        }
    }

    #[allow(dead_code)]
    pub fn step_once(&mut self) -> u32 {
        let opcode = self.current_opcode();
        let cycles = opcode.run::<CPU>(&mut self.cpu, &mut self.memory);
        cycles
    }

    #[allow(dead_code)]
    pub fn get_register_16(&self, register: RegisterLabel16) -> u16 {
        self.cpu.read_16_bits(register)
    }

    #[allow(dead_code)]
    pub fn get_register_8(&self, register: RegisterLabel8) -> u8 {
        self.cpu.read_8_bits(register)
    }

    #[allow(dead_code)]
    pub fn set_register_16(&mut self, register: RegisterLabel16, value: u16) {
        self.cpu.write_16_bits(register, value);
    }

    #[allow(dead_code)]
    pub fn set_register_8(&mut self, register: RegisterLabel8, value: u8) {
        self.cpu.write_8_bits(register, value);
    }

    #[allow(dead_code)]
    pub fn set_flag(&mut self, flag: Flags, set: bool) {
        write_flag::<CPU>(&mut self.cpu, flag, set);
    }

    #[allow(dead_code)]
    pub fn get_flag(&self, flag: Flags) -> bool {
        read_flag::<CPU>(&self.cpu, flag)
    }

    #[allow(dead_code)]
    pub fn set_memory_at(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    #[allow(dead_code)]
    pub fn get_memory_at(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    #[allow(dead_code)]
    pub fn get_current_instruction(&self) -> String {
        let opcode = self.current_opcode();
        opcode.to_string().trim().to_owned()
    }

    #[allow(dead_code)]
    pub fn get_instruction_offset(&self, offset: u16) -> Result<String, ()> {
        let current_counter = self.cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        let desired_counter = current_counter + offset;
        if desired_counter >= self.memory.len() as u16 {
            return Err({});
        } else {
            return Ok(opcodes::decode_instruction(desired_counter, &self.memory)
                .unwrap()
                .to_string()
                .trim()
                .to_owned());
        }
    }

    fn current_opcode(&self) -> OpCode {
        let counter = self.cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        let opcode = opcodes::decode_instruction(counter, &self.memory).unwrap();
        opcode
    }
}
