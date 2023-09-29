mod instruction;
mod layout;
mod opcode_writer;
mod registers;
mod widgets;

use crate::Gameboy;
use layout::Layout;
pub use opcode_writer::OpcodeWriter;
use std::io;
use widgets::{AudioWidget, Channel, FlagsWidget, MemoryWidget, OpCodeWidget, RegistersWidget};

pub enum DebugControls {
    Tick,
    Continue,
}

pub fn update(gb: &Gameboy, breakpoints: &mut Vec<u16>) -> DebugControls {
    // Clear the screen
    print!("{}[2J", 27 as char);

    default_view(gb);

    loop {
        println!("Step? (h for help) [s]");
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Input failed unexpectedly");
        let trimmed = text.trim();
        match trimmed {
            "s" => return DebugControls::Tick,
            "c" => return DebugControls::Continue,
            "b" => breakpoint_menu(breakpoints),
            "h" => print_help(),
            "m" => request_address(gb),
            "o" => default_view(gb),
            "a" => audio_view(gb),
            _ => return DebugControls::Tick,
        }
    }
}

fn default_view(gb: &Gameboy) {
    let opcodes = OpCodeWidget::new(gb);
    let registers = RegistersWidget::new(gb);
    let flags = FlagsWidget::new(gb);
    let mut layout = Layout::new();
    layout.add_widget(Box::new(opcodes), 0);
    layout.add_widget(Box::new(registers), 1);
    layout.add_widget(Box::new(flags), 1);
    layout.draw();
}

fn audio_view(gb: &Gameboy) {
    let channel1 = AudioWidget::new(gb, Channel::One);
    let channel2 = AudioWidget::new(gb, Channel::Two);
    let channel3 = AudioWidget::new(gb, Channel::Three);
    let channel4 = AudioWidget::new(gb, Channel::Four);
    let mut layout = Layout::new();
    layout.add_widget(Box::new(channel1), 0);
    layout.add_widget(Box::new(channel2), 1);
    layout.add_widget(Box::new(channel3), 0);
    layout.add_widget(Box::new(channel4), 1);
    layout.draw();
}

fn breakpoint_menu(breakpoints: &mut Vec<u16>) {
    print_breakpoint_help();

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Input failed unexpectedly");

    let trimmed = text.trim();

    if trimmed == "s" {
        breakpoints.iter().for_each(|bp| println!("{:#X}", *bp));
    } else if trimmed.starts_with('a') {
        // Get the last part of the command
        let address = trimmed.split(' ').collect::<Vec<&str>>()[1];
        println!("{}", address);
        let bp_address = u16::from_str_radix(address, 16).unwrap();
        breakpoints.push(bp_address);
    } else if trimmed.starts_with('r') {
        // Get the last part of the command
        let address = trimmed.split(' ').collect::<Vec<&str>>()[1];
        let bp_address = u16::from_str_radix(address, 16).unwrap();
        breakpoints.retain(|bp| *bp != bp_address);
    }
}

fn print_breakpoint_help() {
    println!("s => show breakpoints");
    println!("a u16 => add breakpoint (address)");
    println!("r u16 => remove breakpoint (address)");
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
    let memory_widget = MemoryWidget::new(gb, address);
    layout.add_widget(Box::new(memory_widget), 0);
    layout.draw();
}

fn print_help() {
    println!("t => tick");
    println!("m => show memory");
    println!("c => continue");
    println!("b => breakpoint menu");
    println!("o => re-print opcodes");
    println!("a => audio");
    println!("h => help");
    println!("Ctrl+c => quit");
}
