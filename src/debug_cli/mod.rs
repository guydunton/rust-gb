mod instruction;
mod layout;
mod registers;
mod widgets;

use crate::Gameboy;
use layout::Layout;
use std::io;
use widgets::{FlagsWidget, OpCodeWidget, RegistersWidget, MemoryWidget};

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
        println!("Continue? (h for help) [c]");
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Input failed unexpectedly");
        let trimmed = text.trim();
        match trimmed.as_ref() {
            "c" => break,
            "h" => print_help(),
            "m" => request_address(gb),
            _ => break,
        }
    }
}

fn request_address(gb: &Gameboy) {
    // Get the address from the user
    println!("Enter start address in hex: ");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Input failed unexpectedly");

    let trimmed = text.trim();

    // Convert address from string to u16
    let address = u16::from_str_radix(trimmed, 16).unwrap();

    // Print the address in a new layout
    let mut layout = Layout::new();
    let memory_widget = MemoryWidget::new(&gb, address);
    layout.add_widget(Box::new(memory_widget), 0);
    layout.draw();
}

fn print_help() {
    println!("c => continue");
    println!("m => show memory");
    println!("h => help");
}
