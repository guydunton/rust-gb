use super::ScreenColor;
use super::memory_view::MemoryView;

pub struct PPU {}

impl PPU {
    pub fn new() -> PPU {
        PPU {}
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

        let mut pixels = [ScreenColor::White; 8];

        let black_bits = byte1 & byte2;

        let bit_mask = byte1 | byte2;

        let light_bits = bit_mask ^ byte2;
        let dark_bits = bit_mask ^ byte1;

        for i in 0..8 {
            if ((black_bits >> i) & 1) == 1 {
                pixels[7 - i] = ScreenColor::Black;
            } else if (light_bits >> i) & 1 == 1 {
                pixels[7 - i] = ScreenColor::Light;
            } else if (dark_bits >> i) & 1 == 1 {
                pixels[7 - i] = ScreenColor::Dark;
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
            let index = mem_view.get_memory_at(0x9800 + map_index as u16);

            // For each point check the tile at that index
            let sprite_data = mem_view.get_memory_slice_at(0x8000 + (index as u16 * 16), 16);

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
}
