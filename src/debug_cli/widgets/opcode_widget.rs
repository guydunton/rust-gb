use super::super::instruction::Instruction;
use super::super::layout::Print;
use crate::gameboy::Gameboy;
use crate::gameboy::RegisterLabel16;

pub struct OpCodeWidget<'a> {
    gb: &'a Gameboy<'a>,
}

impl<'a> OpCodeWidget<'a> {
    pub fn new(gb: &'a Gameboy) -> OpCodeWidget<'a> {
        OpCodeWidget { gb }
    }

    /// Print the first x instructions until can't decode
    pub fn print_instructions(&self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let mut count = 0;
        loop {
            let opcode = self.gb.get_opcode_with_offset(count);
            instructions.push(match opcode {
                Ok((opcode, address)) => Instruction { address, opcode },
                Err(_) => Instruction {
                    address: 0x00,
                    opcode: String::from("unknown instruction"),
                },
            });
            count += 1;

            if count > 20 {
                break;
            }
        }

        instructions
    }

    pub fn get_pc(&self) -> u16 {
        self.gb.get_register_16(RegisterLabel16::ProgramCounter)
    }
}

impl<'a> Print for OpCodeWidget<'a> {
    fn print(&self) -> Vec<String> {
        let instructions = self.print_instructions();

        let mut output = Vec::new();

        let pc_width = 3;

        output.push("-------------------------------------------".to_string());
        output.push(format!(
            "{:<pc_width$} {:<width$} : {}",
            "",
            "Address",
            "Opcode",
            pc_width = pc_width,
            width = 10
        ));
        output.push("-------------------------------------------".to_string());
        for instruction in instructions {
            let pc_counter = if instruction.get_address() == self.get_pc() {
                "->"
            } else {
                "  "
            };

            output.push(format!(
                "{:<#pc_width$} {:<#width$X} : {}",
                pc_counter,
                instruction.get_address(),
                instruction.get_opcode(),
                pc_width = pc_width,
                width = 10
            ));
        }
        output.push("-------------------------------------------".to_string());

        output
    }
}
