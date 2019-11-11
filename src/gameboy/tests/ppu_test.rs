#[cfg(test)]
mod ret_test {
    use crate::gameboy::{Gameboy, Labels, ScreenColor, RegisterLabel8, RegisterLabel16};
    use rust_catch::tests;

    tests! {
        test("A default gameboy will have a white screen") {
            let gb = Gameboy::new(vec![]);

            // Get the VRAM to find the set pixel
            let pixels = gb.get_vram_data();

            assert_eq!(pixels.len(), 256 * 256);
        }

        test("Tiles are displayed correctly") {
            let mut gb = Gameboy::new(vec![]);

            gb.set_memory_at(Labels::BG_PALETTE, 0xE4);

            // Tiles go at 8000
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x10, 0xFF);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x11, 0xFF);

            gb.set_memory_at(Labels::BG_MAP_DATA_1_START, 0x01);

            // Get the VRAM to find the set pixel
            let pixels = gb.get_vram_data();
            let desired_pixels = vec![ScreenColor::Black; 8];
            assert_eq!(pixels[0..8], desired_pixels[..]);
        }

        test("Tile gets drawn the right way around") {
            let mut gb = Gameboy::new(vec![]);

            gb.set_memory_at(Labels::BG_PALETTE, 0xE4);

            // Draw the ® symbol
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x0, 0x3C);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x2, 0x42);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x4, 0xB9);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x6, 0xA5);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x8, 0xB9);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0xA, 0xA5);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0xC, 0x42);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0xE, 0x3C);

            let pixels = gb.get_vram_data();

            let mut tile_pixels = Vec::<Vec<ScreenColor>>::new();

            for line in pixels.chunks(256).take(8) {
                let tile_lines: Vec<ScreenColor> = line[..8].iter().cloned().collect();
                tile_pixels.push(tile_lines);
            }

            assert_eq!(tile_pixels[0], colors(vec![0, 0, 1, 1, 1, 1, 0, 0]));
            assert_eq!(tile_pixels[1], colors(vec![0, 1, 0, 0, 0, 0, 1, 0]));
            assert_eq!(tile_pixels[2], colors(vec![1, 0, 1, 1, 1, 0, 0, 1]));
            assert_eq!(tile_pixels[3], colors(vec![1, 0, 1, 0, 0, 1, 0, 1]));
            assert_eq!(tile_pixels[4], colors(vec![1, 0, 1, 1, 1, 0, 0, 1]));
            assert_eq!(tile_pixels[5], colors(vec![1, 0, 1, 0, 0, 1, 0, 1]));
            assert_eq!(tile_pixels[6], colors(vec![0, 1, 0, 0, 0, 0, 1, 0]));
            assert_eq!(tile_pixels[7], colors(vec![0, 0, 1, 1, 1, 1, 0, 0]));
        }

        test("A tile with multiple shades comes out correctly") {
            let mut gb = Gameboy::new(vec![]);

            // Write away the colors for the tile
            gb.set_memory_at(Labels::CHARACTER_RAM_START, 0x55);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 1, 0x33);

            gb.set_memory_at(Labels::BG_PALETTE, 0xE4);

            let vram : Vec<ScreenColor> = gb.get_vram_data().into_iter().take(8).collect();
            assert_eq!(vram, colors(vec![0, 1, 2, 3, 0, 1, 2, 3]));
        }

        test("Setting the palette color sets the colors of the sprites") {
            let mut gb = Gameboy::new(vec![]);

            // BG palette is FF47
            gb.set_memory_at(Labels::BG_PALETTE, 0xFC);

            // This will set the palette to:
            // 1111_1100
            // Which corresponds to:
            // Color 0 => White
            // Color 1 => Black
            // Color 2 => Black
            // Color 3 => Black

            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x0, 0x50);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x1, 0x30);

            // This sprite is:
            // 0, 1, 2, 3
            // So the resulting pixels should be WHITE, BLACK, BLACK, BLACK

            let vram_pixels : Vec<ScreenColor> = gb.get_vram_data().into_iter().take(4).collect();

            assert_eq!(vram_pixels, colors(vec![0, 3, 3, 3]));
        }

        test("Use instruction to load a palette") {
            // A contains 0xE4
            // HL contains FF47
            // LD (HL), A
            let mut gb = Gameboy::new(vec![0x77]);
            gb.set_register_8(RegisterLabel8::A, 0xE4);
            gb.set_register_16(RegisterLabel16::HL, Labels::BG_PALETTE);

            gb.step_once();

            // The palette should now be set

            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x0, 0x50);
            gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x1, 0x30);

            // This sprite is:
            // 0, 1, 2, 3
            // So the resulting pixels should be WHITE, LIGHT, DARK, BLACK

            let vram_pixels : Vec<ScreenColor> = gb.get_vram_data().into_iter().take(4).collect();

            assert_eq!(vram_pixels, colors(vec![0, 1, 2, 3]));
        }
    }

    fn colors(cols: Vec<i32>) -> Vec<ScreenColor> {
        cols.iter()
            .map(|val| match val {
                0 => ScreenColor::White,
                1 => ScreenColor::Light,
                2 => ScreenColor::Dark,
                3 => ScreenColor::Black,
                _ => ScreenColor::White,
            })
            .collect::<Vec<ScreenColor>>()
    }
}
