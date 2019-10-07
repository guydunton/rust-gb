use super::instrumentation::{get_pc, print_instructions};
use crate::gameboy::Gameboy;
use crate::layout::Print;

pub struct OpCodeWidget<'a> {
    gb: &'a Gameboy,
}

impl<'a> OpCodeWidget<'a> {
    pub fn new(gb: &'a Gameboy) -> OpCodeWidget {
        OpCodeWidget { gb }
    }
}

impl<'a> Print for OpCodeWidget<'a> {
    fn print(&self) -> Vec<String> {
        let instructions = print_instructions(&self.gb);

        let mut output = Vec::new();

        let pc_width = 3;

        output.push(format!("-------------------------------------------"));
        output.push(format!(
            "{:<pc_width$} {:<width$} : {}",
            "",
            "Address",
            "Opcode",
            pc_width = pc_width,
            width = 10
        ));
        output.push(format!("-------------------------------------------"));
        for instruction in instructions {
            let pc_counter = if instruction.get_address() == get_pc(&self.gb) {
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
        output.push(format!("-------------------------------------------"));

        output
    }
}