use super::audio::ALU;
use super::cpu::CPU;
use super::memory_adapter::MemoryAdapter;
use super::memory_labels::Labels;
use super::memory_view::MemoryView;
use super::opcodes::Decoder;
use super::ppu::PPU;
use super::screen::ScreenColor;
use super::{read_flag, write_flag, Flags, OpCode, RegisterLabel16, RegisterLabel8};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TickResult {
    HitBreakpoint,
    FrameComplete,
}

pub struct Gameboy<'a> {
    cpu: CPU,
    ppu: PPU,
    alu: ALU<'a>,
    memory: Vec<u8>,
    rom_header_data: Vec<u8>,
}

impl<'a> Gameboy<'a> {
    pub fn new_with_bootloader<F>(audio_callback: F, game_data: &[u8]) -> Gameboy<'a>
    where
        F: FnMut(i16) + 'a,
    {
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

        if game_data.len() <= bootloader.len() {
            panic!("Game code not larger than bootloader");
        }

        let mut memory = vec![0x0; 0xFFFF + 1];
        memory[..game_data.len()].clone_from_slice(game_data);
        memory[..bootloader.len()].clone_from_slice(&bootloader[..]);

        let mut rom_header_data = vec![0x0u8; bootloader.len()];
        rom_header_data[..0xFF].clone_from_slice(&game_data[..0xFF]);

        Gameboy {
            cpu: CPU::new(),
            ppu: PPU::new(),
            alu: ALU::new(audio_callback),
            memory,
            rom_header_data,
        }
    }

    /// Construct a new Gameboy.
    ///
    /// The provided Vec to fill the first section of memory,
    /// starting at 0x0000. All other parts of memory will be
    /// set to zero.
    #[allow(dead_code)]
    pub fn new(data: Vec<u8>) -> Gameboy<'a> {
        let mut memory = vec![0; 0xFFFF + 1];
        memory[..data.len()].clone_from_slice(&data[..]);

        Gameboy {
            cpu: CPU::new(),
            ppu: PPU::new(),
            alu: ALU::new(|_| {}),
            memory,
            rom_header_data: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn new_with_audio<'b, F>(data: Vec<u8>, audio_callback: F) -> Gameboy<'b>
    where
        F: FnMut(i16) + 'b,
    {
        let mut memory = vec![0; 0xFFFF + 1];
        memory[..data.len()].clone_from_slice(&data[..]);

        Gameboy {
            cpu: CPU::new(),
            ppu: PPU::new(),
            alu: ALU::new(audio_callback),
            memory,
            rom_header_data: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn tick(&mut self, dt: f64) -> TickResult {
        self.tick_with_breaks(dt, &[])
    }

    #[allow(dead_code)]
    pub fn tick_with_breaks(&mut self, dt: f64, breakpoints: &[u16]) -> TickResult {
        let cycles_to_use = (dt * 4194304f64) as u32;
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
        TickResult::FrameComplete
    }

    /// Run the next instruction and return the number of cycles used.
    #[allow(dead_code)]
    pub fn step_once(&mut self) -> u32 {
        // If interrupts are enabled check each interrupt flag
        if self.cpu.is_interrupts_enabled() {
            let interrupt_enabled_flags = self.memory[0xFFFF];
            let interrupt_triggered_flags = self.memory[0xFF0F];

            // Check vblank interrupt
            let vblank_enabled = (interrupt_enabled_flags & 0b0000_0001) > 0;
            let vblank_triggered = (interrupt_triggered_flags & 0b0000_0001) > 0;

            if vblank_enabled && vblank_triggered {
                // if one is enabled call it's handler using 20 CPU cycles
                let return_address = self.cpu.read_16_bits(RegisterLabel16::ProgramCounter);
                let stack_address = self.cpu.read_16_bits(RegisterLabel16::StackPointer);

                let return_addr_bytes = return_address.to_le_bytes();

                self.memory[(stack_address.checked_sub(1).unwrap_or(0)) as usize] =
                    return_addr_bytes[1];
                self.memory[(stack_address.checked_sub(2).unwrap_or(0)) as usize] =
                    return_addr_bytes[0];

                self.cpu.write_16_bits(
                    RegisterLabel16::StackPointer,
                    stack_address.saturating_sub(2),
                );
                self.cpu
                    .write_16_bits(RegisterLabel16::ProgramCounter, 0x40);

                // Reset vblank interrupt trigger
                self.memory[0xFF0F] = self.memory[0xFF0F] & 0b1111_1110;

                // disable interrupts in the process
                self.cpu.disable_interrupts();

                return 20;
            }
        }

        let opcode = self.get_opcode();
        match opcode {
            Ok(op) => {
                let interrupts_enabled_before = self.cpu.is_interrupt_enable_started();

                let mut enable_rom_header = false;

                let cycles;
                {
                    // Set up the memory callbacks
                    let mut mem_adapter = MemoryAdapter::new(&mut self.memory);
                    let ppu_ref = &mut self.ppu;
                    mem_adapter.add_callback(Labels::BG_PALETTE, |new_palette| {
                        ppu_ref.reset_bg_palette(new_palette);
                    });
                    mem_adapter.add_callback(Labels::BOOTLOADER_DISABLE, |_| {
                        // Restore the Cart memory in place of the bootloader
                        enable_rom_header = true;
                    });
                    cycles = op.run(&mut self.cpu, mem_adapter);
                }

                if enable_rom_header {
                    self.memory[..0xFF].copy_from_slice(&self.rom_header_data[..0xFF]);
                }

                // If interrupts are also enabled afterwards then enable interrupts
                if self.cpu.is_interrupt_enable_started() && interrupts_enabled_before {
                    self.cpu.enable_interrupts();
                }

                // Now run the PPU by the same amount of cycles
                self.ppu.tick(cycles, &mut self.memory);

                // Run the ALU by the same amount of cycles
                self.alu.tick(cycles, &mut self.memory);

                cycles
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
        write_flag(&mut self.cpu, flag, set);
    }

    #[allow(dead_code)]
    pub fn get_flag(&self, flag: Flags) -> bool {
        read_flag(&self.cpu, flag)
    }

    #[allow(dead_code)]
    pub fn set_memory_at(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;

        if address == Labels::BG_PALETTE {
            self.ppu.reset_bg_palette(value);
        }
        if address == Labels::BOOTLOADER_DISABLE {
            // Restore the Cart memory in place of the bootloader
            self.memory[..0xFF].copy_from_slice(&self.rom_header_data[..0xFF]);
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

    #[allow(clippy::needless_return)]
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
                        return Err(());
                    }
                    let opcode = self.get_opcode_at(value);
                    match opcode {
                        Ok(op) => opcode_size_offset += op.size(),
                        Err(_err) => return Err(()),
                    }
                }
                None => {
                    return Err(());
                }
            }
        }

        let desired_counter = current_counter.checked_add(opcode_size_offset);

        match desired_counter {
            Some(value) => {
                if value == u16::max_value() {
                    return Err(());
                }
                let opcode = self.get_opcode_at(value);
                return opcode
                    .map(|op| (op.to_string().trim().to_owned(), value))
                    .map_err(|_| ({}));
            }
            None => {
                return Err(());
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_ime_flag(&self) -> bool {
        self.cpu.is_interrupts_enabled()
    }

    fn get_opcode(&self) -> Result<OpCode, String> {
        let counter = self.cpu.read_16_bits(RegisterLabel16::ProgramCounter);
        self.get_opcode_at(counter)
    }

    fn get_opcode_at(&self, address: u16) -> Result<OpCode, String> {
        Decoder::decode_instruction(address, &self.memory)
    }
}
