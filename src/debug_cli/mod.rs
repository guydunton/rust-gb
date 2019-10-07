mod instruction;
mod layout;
mod registers;
mod widgets;

use crate::Gameboy;
use layout::Layout;
use std::io;
use widgets::{FlagsWidget, OpCodeWidget, RegistersWidget};

pub fn update(gb: &Gameboy) {
    // Clear the screen
    print!("{}[2J", 27 as char);

    {
        let opcodes = OpCodeWidget::new(&gb);
        let registers = RegistersWidget::new(&gb);
        let flags = FlagsWidget::new(&gb);
        let mut layout = Layout::new();
        layout.add_widget(Box::new(opcodes), 0);
        layout.add_widget(Box::new(registers), 1);
        layout.add_widget(Box::new(flags), 1);
        layout.draw();
    }

    loop {
        println!("Continue? (h for help)");
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Input failed unexpectadly");
        let trimmed = text.trim();
        match trimmed.as_ref() {
            "c" => break,
            "h" => print_help(),
            _ => print_help(),
        }
    }
}

fn print_help() {
    println!("c => continue");
    println!("h => help");
}
