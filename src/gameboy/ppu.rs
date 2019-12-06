use super::memory_view::MemoryView;
use super::Labels;
use super::ScreenColor;

pub struct PPU {
    bg_palette: [ScreenColor; 4],
    cycles: u32,
}

fn convert_base_to_color(palette_base: u8) -> [ScreenColor; 4] {
    let mut bg_palette = [
        ScreenColor::White,
        ScreenColor::White,
        ScreenColor::White,
        ScreenColor::White,
    ];

    for i in 0..4 {
        match (palette_base >> (i * 2)) & 0b0000_0011 {
            0 => bg_palette[i] = ScreenColor::White,
            1 => bg_palette[i] = ScreenColor::Light,
            2 => bg_palette[i] = ScreenColor::Dark,
            3 => bg_palette[i] = ScreenColor::Black,
            _ => {
                panic!("An error ocurred during palette creation");
            }
        }
    }

    bg_palette
}

impl PPU {
    pub fn new() -> PPU {
        // Create the palette
        let palette_base = 0;
        let bg_palette = convert_base_to_color(palette_base);

        PPU {
            bg_palette,
            cycles: 0,
        }
    }

    pub fn byte_to_colors(&self, byte1: u8, byte2: u8) -> [ScreenColor; 8] {
        // 0b1010_1010
        // 0b0101_0110
        // Results in:
        //   1212_1230

        // e.g.
        // [0,0] => 0
        // [1,0] => 1
        // [0,1] => 2
        // [1,1] => 3

        // Set to color 0
        let mut pixels = [self.bg_palette[0]; 8];

        let black_bits = byte1 & byte2;

        let bit_mask = byte1 | byte2;

        let light_bits = bit_mask ^ byte2;
        let dark_bits = bit_mask ^ byte1;

        for i in 0..8 {
            if ((black_bits >> i) & 1) == 1 {
                pixels[7 - i] = self.bg_palette[3];
            } else if (dark_bits >> i) & 1 == 1 {
                pixels[7 - i] = self.bg_palette[2];
            } else if (light_bits >> i) & 1 == 1 {
                pixels[7 - i] = self.bg_palette[1];
            }
        }

        pixels
    }

    pub fn get_vram_data(&self, memory: &Vec<u8>) -> Vec<ScreenColor> {
        let mem_view = MemoryView::new(&memory);
        let mut vram = vec![ScreenColor::White; 256 * 256];

        // Loop through $9800-$9BFF - BG Map Data 1 to see all the sprites on screen
        for map_index in 0..1024 {
            // Get the value in vram for this index
            let index = mem_view.get_memory_at(Labels::BG_MAP_DATA_1_START + map_index as u16);

            // For each point check the tile at that index
            let sprite_data =
                mem_view.get_memory_slice_at(Labels::CHARACTER_RAM_START + (index as u16 * 16), 16);

            // Render the sprite into the VRAM
            for i in 0..8 {
                let colors = self.byte_to_colors(sprite_data[i * 2], sprite_data[i * 2 + 1]);
                let x_offset = (map_index % 32) * 8;
                let y_offset = (map_index / 32) * 8 * 256;
                let sprite_line_offset = i * 256;
                let start_of_line = x_offset + sprite_line_offset + y_offset;

                vram[start_of_line..(start_of_line + 8)].clone_from_slice(&colors);
            }
        }

        vram
    }

    pub fn reset_bg_palette(&mut self, value: u8) {
        self.bg_palette = convert_base_to_color(value);
    }

    pub fn tick(&mut self, cycles: u32, memory: &mut Vec<u8>) {
        // Get the 7th bit
        let bit_7_set = (memory[Labels::V_BLANK as usize] & 0b1000_0000) != 0;
        let is_screen_on = bit_7_set;

        if is_screen_on {
            let new_cycles = self.cycles + cycles;

            if new_cycles >= 456 {
                // increment the LY register
                memory[Labels::LCDC_Y as usize] = (memory[Labels::LCDC_Y as usize] + 1) % 154;
            }

            self.cycles = new_cycles % 456;
        } else {
            // Reset the LY register
            memory[Labels::LCDC_Y as usize] = 0;
        }
    }
}
