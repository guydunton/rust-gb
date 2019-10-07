use super::instrumentation::{print_flags};
use crate::gameboy::Gameboy;
use crate::layout::Print;

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

        output.push(String::from("----------------"));

        output
    }
}
