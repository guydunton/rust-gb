mod argument;
mod bit;
mod call;
mod category;
mod cb_opcodes;
mod cp;
mod dec;
mod inc;
mod jmp;
mod ld16;
mod ld8;
mod opcodes;
mod pop;
mod push;
mod ret;
mod rotate_left;
mod rotate_left_a;
mod rotate_method;
mod sub;
mod xor;

use super::memory_adapter::MemoryAdapter;
use super::read_write_register::ReadWriteRegister;
use super::{RegisterLabel16, RegisterLabel8};
use argument::{arg_from_str, size_in_bytes, Argument};
use category::{category_from_str, category_size, Category};
use cb_opcodes::CB_DICTIONARY;
use opcodes::DICTIONARY;
use std::fmt;

enum DecodingError {
    CBFailure,
    DefaultCodeFailure,
}

fn parts_from_dictionary(
    code: u8,
    dictionary: &'static Vec<(u8, Vec<&'static str>)>,
    error: DecodingError,
) -> Result<&std::vec::Vec<&str>, DecodingError> {
    dictionary
        .iter()
        .find(|(c, _)| *c == code)
        .ok_or(error)
        .map(|(_, parts)| parts)
}

pub fn decode_instruction(program_counter: u16, program_code: &[u8]) -> Result<OpCode, String> {
    let code = program_code[program_counter as usize];
    let parts_or_error = match code {
        0xCB => {
            // Get the next code
            let cb_code = program_code[program_counter as usize + 1];
            parts_from_dictionary(cb_code, &CB_DICTIONARY, DecodingError::CBFailure)
        }
        _ => {
            // Try to get the value from the dictionary
            parts_from_dictionary(code, &DICTIONARY, DecodingError::DefaultCodeFailure)
        }
    };

    let parts = parts_or_error.map_err(|err_type| match err_type {
        DecodingError::DefaultCodeFailure => format!(
            "Unknown command {:#X} at address: {:#X}",
            code, program_counter
        ),
        DecodingError::CBFailure => format!("Unknown command 0xCB {:#X}", code),
    })?;

    let category = category_from_str(parts[0]);

    let args = parts[1..]
        .iter()
        .map(|arg| arg_from_str(arg, program_counter, program_code));

    let mut clean_args = [Argument::None; 2];

    // Loop through all the arguments and return any errors
    for (i, arg) in args.enumerate() {
        clean_args[i] = arg?;
    }

    Ok(OpCode::new(category, clean_args))
}

pub struct OpCode {
    category: Category,
    args: [Argument; 2],
}

impl OpCode {
    pub fn run<T: ReadWriteRegister>(
        &self,
        cpu: &mut dyn ReadWriteRegister,
        mut memory: MemoryAdapter,
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
                cycles += self.run_ld16::<T>(cpu, memory.get_memory());
            }
            Category::LD8 => {
                cycles += self.run_ld8::<T>(cpu, &mut memory);
            }
            Category::NOP => {
                // Do nothing
                cycles += 4;
            }
            Category::XOR => {
                cycles += self.run_xor::<T>(cpu, memory.get_memory());
            }
            Category::BIT => {
                cycles += self.run_bit::<T>(cpu, memory.get_memory());
            }
            Category::JR => {
                cycles += self.run_jmp::<T>(cpu, memory.get_memory());
            }
            Category::CALL => {
                cycles += self.run_call::<T>(cpu, memory.get_memory());
            }
            Category::RET => {
                cycles += self.run_ret::<T>(cpu, memory.get_memory());
            }
            Category::PUSH => {
                cycles += self.run_push::<T>(cpu, memory.get_memory());
            }
            Category::POP => {
                cycles += self.run_pop::<T>(cpu, memory.get_memory());
            }
            Category::INC => {
                cycles += self.run_inc::<T>(cpu, memory.get_memory());
            }
            Category::DEC => {
                cycles += self.run_dec::<T>(cpu, memory.get_memory());
            }
            Category::RL => {
                cycles += self.run_rl::<T>(cpu, memory.get_memory());
            }
            Category::RLA => {
                cycles += self.run_rla::<T>(cpu, memory.get_memory());
            }
            Category::SUB => {
                cycles += self.run_sub::<T>(cpu, memory.get_memory());
            }
            Category::CP => {
                cycles += self.run_cp::<T>(cpu, memory.get_memory());
            }
        };

        cycles
    }

    pub fn new(category: Category, args: [Argument; 2]) -> OpCode {
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
