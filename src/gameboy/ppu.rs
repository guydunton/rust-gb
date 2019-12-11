use super::memory_view::MemoryView;
use super::Labels;
use super::ScreenColor;

pub struct PPU {
    screen_data: Vec<ScreenColor>,
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
        let screen_data = vec![ScreenColor::White; 160 * 144];

        PPU {
            screen_data,
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

    pub fn get_screen_data(&self) -> &Vec<ScreenColor> {
        &self.screen_data
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

                if memory[Labels::LCDC_Y as usize] <= 144 {
                    // Write a line into the screen data starting at LCDC_Y - 1
                    
                    // Which line are we drawing
                    let drawing_line = memory[Labels::LCDC_Y as usize].saturating_sub(1);

                    // This would be where we pick which pixels we want from VRAM

                    // Find the screen x & screen y
                    let screen_origin_x = memory[Labels::SCROLL_X as usize] as u16;
                    let screen_origin_y = memory[Labels::SCROLL_Y as usize] as u16;

                    // for each pixel in line
                    for pixel in 0..160 {
                        // Find the coord in the screen data we are writing
                        let pixel_index = pixel + drawing_line as u32 * 160;

                        // Find the pixel in vram
                        let vram_x = ((screen_origin_x + pixel as u16) % 255) as u32;
                        let vram_y = ((screen_origin_y + drawing_line as u16) % 255) as u32;

                        let tile_index = find_tile_index(vram_x, vram_y);
                        let tile_bytes = get_tile_data(tile_index, memory);

                        // Find the pixel within the tile that the screen is looking at
                        let inside_tile_x = (vram_x % 8) as u8;
                        let inside_tile_y = (vram_y % 8) as u8;

                        let pixel_value = get_pixel_value_from_sprite(inside_tile_x, inside_tile_y, &tile_bytes);
                        let pixel_color = self.bg_palette[pixel_value as usize];
                        self.screen_data[pixel_index as usize] = pixel_color;
                    }
                }
            }

            self.cycles = new_cycles % 456;
        } else {
            // Reset the LY register
            memory[Labels::LCDC_Y as usize] = 0;
        }
    }
}

fn find_tile_index(vram_x: u32, vram_y: u32) -> u16 {
    // Find which tile it is
    let tile_x = vram_x / 8;
    let tile_y = vram_y / 8;

    // Find the tile index
    let tile_index = (tile_x + tile_y * 32) as u16;
    tile_index
}

fn get_tile_data(tile_index: u16, memory: &Vec<u8>) -> &[u8] {
    // Get the tile_data_start
    let tile_data_start = (Labels::CHARACTER_RAM_START + 
        memory[(Labels::BG_MAP_DATA_1_START + tile_index) as usize] as u16 * 16) as usize;

    let tile_bytes = &memory[tile_data_start..(tile_data_start + 16)];
    tile_bytes
}
 
fn get_pixel_value_from_sprite(x: u8, y: u8, sprite_data: &[u8]) -> u8 {
    let row_bytes: [u8;2] = [
        sprite_data[y as usize * 2],
        sprite_data[y as usize * 2 + 1]
    ];

    // Get the least significant bit
    let ls_bit = (row_bytes[0] >> (7-x)) & 1;
    let ms_bit = (row_bytes[1] >> (7-x)) & 1;

    ls_bit | (ms_bit << 1)
}

#[test]
fn get_the_correct_pixel_from_background_sprite() {
    // Sprite Data
    let sprite_data = [
        0x55, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];

    // Get the correct color
    assert_eq!(get_pixel_value_from_sprite(0, 0, &sprite_data), 0);
    assert_eq!(get_pixel_value_from_sprite(1, 0, &sprite_data), 1);
    assert_eq!(get_pixel_value_from_sprite(2, 0, &sprite_data), 2);
    assert_eq!(get_pixel_value_from_sprite(3, 0, &sprite_data), 3);
}
