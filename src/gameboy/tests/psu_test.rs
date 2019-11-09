#[cfg(test)]
mod ret_test {
    use crate::gameboy::screen::ScreenColor;
    use crate::gameboy::Gameboy;
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

            // Tiles go at 8000
            gb.set_memory_at(0x8010, 0xFF);
            gb.set_memory_at(0x8011, 0xFF);

            gb.set_memory_at(0x9800, 0x01);

            // Get the VRAM to find the set pixel
            let pixels = gb.get_vram_data();
            let desired_pixels = vec![ScreenColor::Black; 8];
            assert_eq!(pixels[0..8], desired_pixels[..]);
        }

        test("Tile gets drawn the right way around") {
            let mut gb = Gameboy::new(vec![]);

            // Draw the ® symbol
            gb.set_memory_at(0x8000, 0x3C);
            gb.set_memory_at(0x8002, 0x42);
            gb.set_memory_at(0x8004, 0xB9);
            gb.set_memory_at(0x8006, 0xA5);
            gb.set_memory_at(0x8008, 0xB9);
            gb.set_memory_at(0x800A, 0xA5);
            gb.set_memory_at(0x800C, 0x42);
            gb.set_memory_at(0x800E, 0x3C);

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
