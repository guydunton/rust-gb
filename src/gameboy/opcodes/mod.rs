mod argument;
mod bit;
mod call;
mod catagory;
mod dec;
mod inc;
mod jmp;
mod ld16;
mod ld8;
mod pop;
mod push;
mod rotate_left;
mod rotate_left_a;
mod rotate_method;
mod xor;

use super::read_write_register::ReadWriteRegister;
use super::{RegisterLabel16, RegisterLabel8};
use argument::{arg_from_str, size_in_bytes, Argument};
use catagory::{catagory_from_str, catagory_size, Catagory};
use std::fmt;

pub fn decode_instruction(program_counter: u16, program_code: &[u8]) -> Result<OpCode, String> {
    let code = program_code[program_counter as usize];

    let opcode = |text: &str| -> Result<OpCode, String> {
        let parts = text.split(' ').collect::<Vec<&str>>();
        let catagory = catagory_from_str(parts[0]);

        let args = parts[1..]
            .iter()
            .map(|arg| arg_from_str(arg, program_counter, program_code));

        let mut clean_args = Vec::new();
        for arg in args {
            clean_args.push(arg?);
        }

        Ok(OpCode::new(catagory, clean_args))
    };

    match code {
        0x00 => opcode("NOP"),
        0x05 => opcode("DEC B"),
        0x06 => opcode("LD8 B d8"),
        0x0C => opcode("INC C"),
        0x0E => opcode("LD8 C d8"),
        0x11 => opcode("LD16 DE d16"),
        0x17 => Ok(OpCode::new(Catagory::RLA, vec![])),
        0x1A => opcode("LD8 A (DE)"),
        0x20 => opcode("JR NZ r8"),
        0x21 => opcode("LD16 HL d16"),
        0x22 => opcode("LD8 (HL+) A"),
        0x31 => opcode("LD16 SP d16"),
        0x32 => opcode("LD8 (HL-) A"),
        0x3E => opcode("LD8 A d8"),
        0x4F => opcode("LD8 C A"),
        0x77 => opcode("LD8 (HL) A"),
        0xAF => opcode("XOR A"),
        0xC1 => opcode("POP BC"),
        0xC5 => opcode("PUSH BC"),
        0xCB => {
            // 0xCB is prefix and the next byte shows the actual instruction
            let cb_instruction = program_code[program_counter as usize + 1];
            match cb_instruction {
                0x11 => opcode("RL C"),
                0x7C => opcode("BIT 7 H"),
                _ => Err(format!("Unknown command 0xCB {:#X}", cb_instruction)),
            }
        }
        0xCD => opcode("CALL a16"),
        0xE0 => opcode("LD8 (a8) A"),
        0xE2 => opcode("LD8 (C) A"),
        _ => Err(format!(
            "Unknown command {:#X} at address: {:#X}",
            code, program_counter
        )),
    }
}

pub struct OpCode {
    catagory: Catagory,
    args: Vec<Argument>,
}

impl OpCode {
    pub fn run<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        memory: &mut Vec<u8>,
    ) -> u32 {
        // Update the program counter
        let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        cpu.write_16_bits(
            RegisterLabel16::ProgramCounter,
            program_counter + self.size(),
        );

        let mut cycles = 0;

        match self.catagory {
            Catagory::LD16 => {
                cycles += self.run_ld16::<T>(cpu, memory);
            }
            Catagory::LD8 => {
                cycles += self.run_ld8::<T>(cpu, memory);
            }
            Catagory::NOP => {
                // Do nothing
                cycles += 4;
            }
            Catagory::XOR => {
                cycles += self.run_xor::<T>(cpu, memory);
            }
            Catagory::BIT => {
                cycles += self.run_bit::<T>(cpu, memory);
            }
            Catagory::JR => {
                cycles += self.run_jmp::<T>(cpu, memory);
            }
            Catagory::CALL => {
                cycles += self.run_call::<T>(cpu, memory);
            }
            Catagory::PUSH => {
                cycles += self.run_push::<T>(cpu, memory);
            }
            Catagory::POP => {
                cycles += self.run_pop::<T>(cpu, memory);
            }
            Catagory::INC => {
                cycles += self.run_inc::<T>(cpu, memory);
            }
            Catagory::DEC => {
                cycles += self.run_dec::<T>(cpu, memory);
            }
            Catagory::RL => {
                cycles += self.run_rl::<T>(cpu, memory);
            }
            Catagory::RLA => {
                cycles += self.run_rla::<T>(cpu, memory);
            }
        };

        cycles
    }

    pub fn new(catagory: Catagory, args: Vec<Argument>) -> OpCode {
        OpCode { catagory, args }
    }

    pub fn size(&self) -> u16 {
        let type_size = catagory_size(self.catagory);
        self.args.iter().map(|arg| size_in_bytes(*arg)).sum::<u16>() + type_size
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let catagory = format!("{:?}", self.catagory);

        let args = self
            .args
            .iter()
            .map(|arg| format!("{}", arg))
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{} {}", catagory, args)
    }
}
