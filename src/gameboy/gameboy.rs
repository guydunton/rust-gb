use super::cpu::CPU;
use super::memory_adapter::MemoryAdapter;
use super::memory_labels::Labels;
use super::memory_view::MemoryView;
use super::opcodes::decode_instruction;
use super::ppu::PPU;
use super::read_write_register::ReadWriteRegister;
use super::screen::ScreenColor;
use super::OpCode;
use super::RegisterLabel16;
use super::RegisterLabel8;
use super::{read_flag, write_flag, Flags};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TickResult {
    HitBreakpoint,
    FrameComplete,
}

pub struct Gameboy {
    cpu: CPU,
    ppu: PPU,
    memory: Vec<u8>,
}

impl Gameboy {
    pub fn new_with_bootloader() -> Gameboy {
        let bootloader = vec![
            0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26,
            0xFF, 0x0E, // 0x10
            0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E,
            0xFC, 0xE0, // 0x20
            0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00,
            0x13, 0x7B, // 0x30
            0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05,
            0x20, 0xF9, // 0x40
            0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32,
            0x0D, 0x20, // 0x50
            0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0,
            0x40, 0x04, // 0x60
            0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D,
            0x20, 0xF2, // 0x70
            0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64,
            0x20, 0x06, // 0x80
            0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2,
            0x05, 0x20, // 0x90
            0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB,
            0x11, 0x17, // 0xA0
            0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D,
            0x00, 0x0B, // 0xB0
            0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89,
            0x00, 0x0E, // 0xC0
            0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E,
            0xEC, 0xCC, // 0xD0
            0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5,
            0x42, 0x3C, // 0xE0
            0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE,
            0x34, 0x20, // 0xF0
            0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01,
            0xE0, 0x50,
        ];

        let fake_header_data = vec![
            0x00, 0x00, 0x00, 0x00, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73,
            0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
            0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E,
            0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];

        let mut data = vec![0x0; bootloader.len() + fake_header_data.len()];
        data[..bootloader.len()].clone_from_slice(&bootloader[..]);
        data[bootloader.len()..(bootloader.len() + fake_header_data.len())]
            .clone_from_slice(&fake_header_data[..]);

        Gameboy::new(data)
    }

    /// Construct a new Gameboy.
    ///
    /// The provided Vec to fill the first section of memory,
    /// starting at 0x0000. All other parts of memory will be
    /// set to zero.
    #[allow(dead_code)]
    pub fn new(data: Vec<u8>) -> Gameboy {
        let mut memory = vec![0; 0xFFFF];
        memory[..data.len()].clone_from_slice(&data[..]);

        Gameboy {
            cpu: CPU::new(),
            ppu: PPU::new(),
            memory,
        }
    }

    #[allow(dead_code)]
    pub fn tick(&mut self, dt: f64, breakpoints: &Vec<u16>) -> TickResult {
        let cycles_to_use = (dt * 1000000f64) as u32;
        let mut total_cycles_used = 0;

        loop {
            total_cycles_used += self.step_once();
            if total_cycles_used > cycles_to_use {
                break;
            }
            if breakpoints
                .iter()
                .any(|bp| *bp == self.cpu.read_16_bits(RegisterLabel16::ProgramCounter))
            {
                return TickResult::HitBreakpoint;
            }
        }
        return TickResult::FrameComplete;
    }

    /// Run the next instruction and return the number of cycles used.
    #[allow(dead_code)]
    pub fn step_once(&mut self) -> u32 {
        let opcode = self.get_opcode();
        match opcode {
            Ok(op) => {
                let cycles;
                {
                    // Set up the memory callbacks
                    let mut mem_adapter = MemoryAdapter::new(&mut self.memory);
                    let ppu_ref = &mut self.ppu;
                    mem_adapter.add_callback(Labels::BG_PALETTE, |new_palette| {
                        ppu_ref.reset_bg_palette(new_palette);
                    });
                    cycles = op.run::<CPU>(&mut self.cpu, mem_adapter);
                }

                // Now run the PPU by the same amount of cycles
                self.ppu.tick(cycles, &mut self.memory);

                return cycles;
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_register_16(&self, register: RegisterLabel16) -> u16 {
        self.cpu.read_16_bits(register)
    }

    #[allow(dead_code)]
    pub fn get_register_8(&self, register: RegisterLabel8) -> u8 {
        self.cpu.read_8_bits(register)
    }

    #[allow(dead_code)]
    pub fn set_register_16(&mut self, register: RegisterLabel16, value: u16) {
        self.cpu.write_16_bits(register, value);
    }

    #[allow(dead_code)]
    pub fn set_register_8(&mut self, register: RegisterLabel8, value: u8) {
        self.cpu.write_8_bits(register, value);
    }

    #[allow(dead_code)]
    pub fn set_flag(&mut self, flag: Flags, set: bool) {
        write_flag::<CPU>(&mut self.cpu, flag, set);
    }

    #[allow(dead_code)]
    pub fn get_flag(&self, flag: Flags) -> bool {
        read_flag::<CPU>(&self.cpu, flag)
    }

    #[allow(dead_code)]
    pub fn set_memory_at(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;

        if address == Labels::BG_PALETTE {
            self.ppu.reset_bg_palette(value);
        }

        // This hack resets any values in the case of the display being switched off
        self.ppu.tick(0, &mut self.memory);
    }

    #[allow(dead_code)]
    pub fn get_memory_at(&self, address: u16) -> u8 {
        MemoryView::new(&self.memory).get_memory_at(address)
    }

    #[allow(dead_code)]
    pub fn get_memory_slice_at(&self, address: u16, size: u16) -> &[u8] {
        MemoryView::new(&self.memory).get_memory_slice_at(address, size)
    }

    /// Return the VRAM information
    ///
    /// Return the data stored in the VRAM as pixel data. This is useful for
    /// viewing all the tiles currently stored
    #[allow(dead_code)]
    pub fn get_vram_data(&self) -> Vec<ScreenColor> {
        self.ppu.get_vram_data(&self.memory)
    }

    #[allow(dead_code)]
    pub fn get_screen_data(&self) -> &Vec<ScreenColor> {
        self.ppu.get_screen_data()
    }

    #[allow(dead_code)]
    pub fn get_current_instruction(&self) -> Option<String> {
        let opcode = self.get_opcode();
        opcode.map(|op| op.to_string().trim().to_owned()).ok()
    }

    #[allow(dead_code)]
    pub fn get_opcode_with_offset(&self, offset: u16) -> Result<(String, u16), ()> {
        let current_counter = self.cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        let mut opcode_size_offset: u16 = 0;
        // Loop through instructions to get the correct instructions
        for _ in 0..offset {
            // decode the instruction at current_counter + opcode_size_offset
            let added_counter = current_counter.checked_add(opcode_size_offset);

            match added_counter {
                Some(value) => {
                    if value == u16::max_value() {
                        return Err({});
                    }
                    let opcode = self.get_opcode_at(value);
                    match opcode {
                        Ok(op) => opcode_size_offset += op.size(),
                        Err(_err) => return Err({}),
                    }
                }
                None => {
                    return Err({});
                }
            }
        }

        let desired_counter = current_counter.checked_add(opcode_size_offset);

        match desired_counter {
            Some(value) => {
                if value == u16::max_value() {
                    return Err({});
                }
                let opcode = self.get_opcode_at(value);
                return opcode
                    .map(|op| (op.to_string().trim().to_owned(), value))
                    .map_err(|_| ({}));
            }
            None => {
                return Err({});
            }
        }
    }

    fn get_opcode(&self) -> Result<OpCode, String> {
        let counter = self.cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        self.get_opcode_at(counter)
    }

    fn get_opcode_at(&self, address: u16) -> Result<OpCode, String> {
        decode_instruction(address, &self.memory)
    }
}
