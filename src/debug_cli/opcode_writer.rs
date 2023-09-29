use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct OpcodeWriter {
    opcode_record: HashMap<u16, String>,
    filename: PathBuf,
}

impl OpcodeWriter {
    pub fn new(filename: &PathBuf) -> Self {
        let mut opcode_record: HashMap<u16, String> = HashMap::new();
        opcode_record.reserve(4000);
        Self {
            opcode_record,
            filename: filename.clone(),
        }
    }

    pub fn store_opcode(&mut self, address: u16, opcode: String) {
        if !self.opcode_record.contains_key(&address) {
            let _ = self.opcode_record.insert(address, opcode);
        }
    }

    pub fn write_file(&mut self) {
        let mut file = File::create(&self.filename).unwrap();
        let mut keys: Vec<u16> = self.opcode_record.keys().map(|val| *val).collect();
        keys.sort();

        for key in keys {
            let opcode = self.opcode_record.entry(key).or_default();
            write!(file, "{:#06x}: {}\n", key, opcode).unwrap();
        }
    }
}
