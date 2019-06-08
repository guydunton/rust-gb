use crate::debug::instrumentation::{get_pc, get_registers, print_flags, print_instructions};
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

pub struct RegistersWidget<'a> {
    gb: &'a Gameboy,
}

impl<'a> RegistersWidget<'a> {
    pub fn new(gb: &'a Gameboy) -> RegistersWidget {
        RegistersWidget { gb }
    }
}

impl<'a> Print for RegistersWidget<'a> {
    fn print(&self) -> Vec<String> {
        let registers = get_registers(&self.gb);
        let register_order = vec![
            "A", "F", "AF", "B", "C", "BC", "D", "E", "DE", "H", "L", "HL", "PC", "SP",
        ];

        let mut output = Vec::new();

        output.push(String::from("----------------"));
        output.push(format!("{:<#width$} : {}", "Register", "Value", width = 8));
        output.push(String::from("----------------"));

        for register in register_order {
            output.push(format!(
                "{:<#width$} : {}",
                register,
                registers.get_register_val(&register.to_string()),
                width = 8
            ));
        }
        output
    }
}

pub struct FlagsWidget<'a> {
    gb: &'a Gameboy,
}

impl<'a> FlagsWidget<'a> {
    pub fn new(gb: &'a Gameboy) -> FlagsWidget {
        FlagsWidget { gb }
    }
}

impl<'a> Print for FlagsWidget<'a> {
    fn print(&self) -> Vec<String> {
        let mut output = Vec::new();

        output.push(String::from("----------------"));
        output.push(format!("{:<#width$} : {}", "Flag", "Set", width = 5));
        output.push(String::from("----------------"));

        for flag in print_flags(self.gb) {
            output.push(flag);
        }

        output
    }
}
