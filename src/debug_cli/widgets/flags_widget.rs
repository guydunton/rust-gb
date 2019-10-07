use super::super::layout::Print;
use crate::gameboy::Gameboy;

pub struct FlagsWidget<'a> {
    gb: &'a Gameboy,
}

impl<'a> FlagsWidget<'a> {
    pub fn new(gb: &'a Gameboy) -> FlagsWidget {
        FlagsWidget { gb }
    }

    pub fn print_flags(&self) -> Vec<String> {
        use crate::gameboy::Flags;
        let flags = vec![Flags::Z, Flags::N, Flags::H, Flags::C];

        let flag_data = flags
            .iter()
            .map(|f| (format!("{:?}", f), f))
            .map(|(label, flag)| {
                (
                    label,
                    match self.gb.get_flag(*flag) {
                        true => "1".to_string(),
                        false => "0".to_string(),
                    },
                )
            })
            .map(|(label, text)| format!("{}: {}", label, text))
            .collect::<Vec<String>>();

        flag_data
    }
}

impl<'a> Print for FlagsWidget<'a> {
    fn print(&self) -> Vec<String> {
        let mut output = Vec::new();

        output.push(String::from("----------------"));
        output.push(format!("{:<#width$} : {}", "Flag", "Set", width = 5));
        output.push(String::from("----------------"));

        for flag in self.print_flags() {
            output.push(flag);
        }

        output.push(String::from("----------------"));

        output
    }
}
