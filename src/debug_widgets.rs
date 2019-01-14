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
        let instructions = self.gb.print_instructions();

        let mut output = Vec::new();

        output.push(format!("-------------------------------------------"));
        output.push(format!("{:<width$} : {}", "Address", "Opcode", width = 10));
        output.push(format!("-------------------------------------------"));
        for instruction in instructions {
            output.push(format!(
                "{:<#width$X} : {}",
                instruction.get_address(),
                instruction.get_opcode(),
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
        let registers = self.gb.get_registers();
        let register_order = vec![
            "A", "F", "AF", "B", "C", "BC", "D", "E", "DE", "H", "L", "HL", "PC", "SP",
        ];

        let mut output = Vec::new();
        for register in register_order {
            output.push(format!(
                "{}: {}",
                register,
                registers.get_register_val(&register.to_string())
            ));
        }
        output
    }
}
