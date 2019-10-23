mod argument;
mod bit;
mod call;
mod category;
mod cb_opcodes;
mod dec;
mod inc;
mod jmp;
mod ld16;
mod ld8;
mod opcodes;
mod pop;
mod push;
mod rotate_left;
mod rotate_left_a;
mod rotate_method;
mod xor;

use super::read_write_register::ReadWriteRegister;
use super::{RegisterLabel16, RegisterLabel8};
use argument::{arg_from_str, size_in_bytes, Argument};
use category::{category_from_str, category_size, Category};
use std::fmt;
use opcodes::code_to_opcode;

pub fn decode_instruction(program_counter: u16, program_code: &[u8]) -> Result<OpCode, String> {
    let code = program_code[program_counter as usize];

    let opcode = |text: &str| -> Result<OpCode, String> {
        let parts = text.split(' ').collect::<Vec<&str>>();
        let category = category_from_str(parts[0]);

        let args = parts[1..]
            .iter()
            .map(|arg| arg_from_str(arg, program_counter, program_code));

        let mut clean_args = Vec::new();
        for arg in args {
            clean_args.push(arg?);
        }

        Ok(OpCode::new(category, clean_args))
    };

    let code_result = code_to_opcode(code, program_counter, program_code);
    opcode(code_result?)

}

pub struct OpCode {
    category: Category,
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

        match self.category {
            Category::LD16 => {
                cycles += self.run_ld16::<T>(cpu, memory);
            }
            Category::LD8 => {
                cycles += self.run_ld8::<T>(cpu, memory);
            }
            Category::NOP => {
                // Do nothing
                cycles += 4;
            }
            Category::XOR => {
                cycles += self.run_xor::<T>(cpu, memory);
            }
            Category::BIT => {
                cycles += self.run_bit::<T>(cpu, memory);
            }
            Category::JR => {
                cycles += self.run_jmp::<T>(cpu, memory);
            }
            Category::CALL => {
                cycles += self.run_call::<T>(cpu, memory);
            }
            Category::PUSH => {
                cycles += self.run_push::<T>(cpu, memory);
            }
            Category::POP => {
                cycles += self.run_pop::<T>(cpu, memory);
            }
            Category::INC => {
                cycles += self.run_inc::<T>(cpu, memory);
            }
            Category::DEC => {
                cycles += self.run_dec::<T>(cpu, memory);
            }
            Category::RL => {
                cycles += self.run_rl::<T>(cpu, memory);
            }
            Category::RLA => {
                cycles += self.run_rla::<T>(cpu, memory);
            }
        };

        cycles
    }

    pub fn new(category: Category, args: Vec<Argument>) -> OpCode {
        OpCode { category, args }
    }

    pub fn size(&self) -> u16 {
        let type_size = category_size(self.category);
        self.args.iter().map(|arg| size_in_bytes(*arg)).sum::<u16>() + type_size
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let category = format!("{:?}", self.category);

        let args = self
            .args
            .iter()
            .map(|arg| format!("{}", arg))
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{} {}", category, args)
    }
}
