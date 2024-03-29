mod audio;
mod cpu;
mod flags_register;

#[allow(clippy::module_inception)]
mod gameboy;
mod interrupt_routine;
mod memory_adapter;
mod memory_labels;
mod memory_view;
mod opcodes;
mod ppu;
mod register;
mod screen;

// Include the gameboy test suite
#[cfg(test)]
mod tests;

// Expose Gameboy, flags, opcodes and registers
pub use self::flags_register::{read_flag, write_flag, Flags};
pub use self::gameboy::{Gameboy, TickResult};
pub use self::memory_labels::Labels;
pub use self::opcodes::OpCode;
pub use self::register::{RegisterLabel16, RegisterLabel8};
pub use self::screen::ScreenColor;
