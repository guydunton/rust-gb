use super::super::layout::Print;
use crate::gameboy::Gameboy;

pub enum Channel {
    One,
    Two,
    Three,
    Four,
}

pub struct AudioWidget<'a> {
    gb: &'a Gameboy<'a>,
    channel: Channel,
}

impl<'a> AudioWidget<'a> {
    pub fn new(gb: &'a Gameboy, channel: Channel) -> AudioWidget<'a> {
        AudioWidget { gb, channel }
    }

    fn print_channel_registers(&self, registers: &[u16], out: &mut Vec<String>) {
        registers
            .iter()
            .map(|register| {
                format!(
                    "{:#X}: {:#010b}",
                    register,
                    self.gb.get_memory_at(*register)
                )
            })
            .for_each(|register| out.push(register));
    }
}

impl<'a> Print for AudioWidget<'a> {
    fn print(&self) -> Vec<String> {
        let mut output = Vec::new();

        match self.channel {
            Channel::One => {
                output.push(String::from("Channel 1 (Tone & Sweep)"));
                self.print_channel_registers(
                    &[0xFF10, 0xFF11, 0xFF12, 0xFF13, 0xFF14],
                    &mut output,
                );
            }
            Channel::Two => {
                output.push(String::from("Channel 2 (Tone)"));
                self.print_channel_registers(&[0xFF16, 0xFF17, 0xFF18, 0xFF19], &mut output);
                output.push(String::from(""));
            }
            Channel::Three => {
                output.push(String::from("Channel 3 (Wave output)"));
                self.print_channel_registers(
                    &[0xFF1A, 0xFF1B, 0xFF1C, 0xFF1D, 0xFF1E],
                    &mut output,
                );
            }
            Channel::Four => {
                output.push(String::from("Channel 4 (Noise)"));
                self.print_channel_registers(&[0xFF20, 0xFF21, 0xFF22, 0xFF23], &mut output);
                output.push(String::from(""));
            }
        }

        output.push(String::from("---------------------------"));

        output
    }
}
