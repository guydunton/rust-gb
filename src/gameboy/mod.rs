mod cpu;
mod flags_register;
mod gameboy;
mod opcodes;
mod read_write_register;
mod register;

// Expose screen because it's not finished yet
pub mod screen;

// Include the gameboy test suite
mod tests;

// Expose Gameboy, flags, opcodes and registers
pub use self::flags_register::{read_flag, write_flag, Flags};
pub use self::gameboy::Gameboy;
pub use self::opcodes::OpCode;
pub use self::register::{RegisterLabel16, RegisterLabel8};
