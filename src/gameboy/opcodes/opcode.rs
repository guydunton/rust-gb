use super::super::cpu::CPU;
use super::super::memory_adapter::MemoryAdapter;
use super::argument::{size_in_bytes, Argument};
use super::category::{category_size, Category};
use super::run_fns::*;
use crate::gameboy::RegisterLabel16;
use std::fmt;

pub struct OpCode {
    category: Category,
    args: [Argument; 2],
}

impl OpCode {
    pub fn run(&self, cpu: &mut CPU, mut memory: MemoryAdapter) -> u32 {
        // Update the program counter
        let program_counter = cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        cpu.write_16_bits(
            RegisterLabel16::ProgramCounter,
            program_counter + self.size(),
        );

        let mut cycles = 0;

        match self.category {
            Category::LD16 => {
                cycles += run_ld16(&self.args, cpu, memory.get_memory());
            }
            Category::LD8 => {
                cycles += run_ld8(&self.args, cpu, &mut memory);
            }
            Category::NOP => {
                // Do nothing
                cycles += 4;
            }
            Category::XOR => {
                cycles += run_xor(&self.args, cpu, memory.get_memory());
            }
            Category::BIT => {
                cycles += run_bit(&self.args, cpu, memory.get_memory());
            }
            Category::JP => {
                cycles += run_jmp(&self.args, cpu, memory.get_memory());
            }
            Category::CALL => {
                cycles += run_call(&self.args, cpu, memory.get_memory());
            }
            Category::RET => {
                cycles += run_ret(cpu, memory.get_memory());
            }
            Category::PUSH => {
                cycles += run_push(&self.args, cpu, memory.get_memory());
            }
            Category::POP => {
                cycles += run_pop(&self.args, cpu, memory.get_memory());
            }
            Category::ADD => {
                cycles += run_add(&self.args, cpu, memory.get_memory());
            }
            Category::INC => {
                cycles += run_inc(&self.args, cpu, memory.get_memory());
            }
            Category::DEC => {
                cycles += run_dec(&self.args, cpu, memory.get_memory());
            }
            Category::RL => {
                cycles += run_rl(&self.args, cpu, memory.get_memory());
            }
            Category::RLA => {
                cycles += run_rla(cpu, memory.get_memory());
            }
            Category::SUB => {
                cycles += run_sub(&self.args, cpu, memory.get_memory());
            }
            Category::CP => {
                cycles += run_cp(&self.args, cpu, memory.get_memory());
            }
            Category::OR => {
                cycles += run_or(&self.args, cpu, memory.get_memory());
            }
            Category::EI => {
                // TODO: Implement interrupts
                cycles += 4;
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