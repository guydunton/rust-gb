use super::super::layout::Print;
use crate::gameboy::Gameboy;

pub struct MemoryWidget<'a> {
    gb: &'a Gameboy,
    start: u16,
}

impl<'a> MemoryWidget<'a> {
    pub fn new(gb: &'a Gameboy, start: u16) -> MemoryWidget {
        MemoryWidget { gb, start }
    }
}

impl<'a> Print for MemoryWidget<'a> {
    fn print(&self) -> Vec<String> {
        let mut output = Vec::new();

        output.push(String::from("----------------"));
        output.push(format!("{:<#width$} : {}", "Addr", "Values", width = 5));
        output.push(String::from("----------------"));

        // Retrieve a slice of memory and print it out in lines
        let memory = self.gb.get_memory_slice_at(self.start, 256);

        let chunk_size = 16;
        for (i, chunk) in memory.chunks(chunk_size).enumerate() {
            let marker = format!("{:#X}", self.start + (i * chunk_size) as u16);
            let bytes: Vec<String> = chunk.iter().map(|val| -> String { return format!("{:02X}", val); }).collect();

            output.push(format!("{}: {}", marker, bytes.join(",")));
        }

        output.push(String::from("----------------"));

        output
    }
}
