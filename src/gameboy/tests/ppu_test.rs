use super::infinite_loop_gb;
use crate::gameboy::{Gameboy, Labels, RegisterLabel16, RegisterLabel8, ScreenColor};

// Default palette:
// 0 => White
// 1 => Light
// 2 => Dark
// 3 => Black
const DEFAULT_PALLETE: u8 = 0b1110_0100;

fn ppu_infinite_loop_gb() -> Gameboy<'static> {
    let mut gb = infinite_loop_gb();

    // Turn the screen on & set the palette
    gb.set_memory_at(Labels::V_BLANK, 0x80);
    gb.set_memory_at(Labels::BG_PALETTE, DEFAULT_PALLETE);

    gb
}

#[test]
fn a_default_gameboy_will_have_a_white_screen() {
    let gb = Gameboy::new(vec![]);

    // Get the VRAM to find the set pixel
    let pixels = gb.get_vram_data();

    assert_eq!(pixels.len(), 256 * 256);
}

#[test]
fn tiles_are_displayed_correctly() {
    let mut gb = Gameboy::new(vec![]);

    gb.set_memory_at(Labels::BG_PALETTE, DEFAULT_PALLETE);

    // Tiles go at 8000
    gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x10, 0xFF);
    gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x11, 0xFF);

    gb.set_memory_at(Labels::BG_MAP_DATA_1_START, 0x01);

    // Get the VRAM to find the set pixel
    let pixels = gb.get_vram_data();
    let desired_pixels = vec![ScreenColor::Black; 8];
    assert_eq!(pixels[0..8], desired_pixels[..]);
}

#[test]
fn tile_gets_drawn_the_right_way_around() {
    let mut gb = Gameboy::new(vec![]);

    gb.set_memory_at(Labels::BG_PALETTE, DEFAULT_PALLETE);

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

#[test]
fn a_tile_with_multiple_shades_comes_out_correctly() {
    let mut gb = Gameboy::new(vec![]);

    // Write away the colors for the tile
    gb.set_memory_at(Labels::CHARACTER_RAM_START, 0x55);
    gb.set_memory_at(Labels::CHARACTER_RAM_START + 1, 0x33);

    gb.set_memory_at(Labels::BG_PALETTE, DEFAULT_PALLETE);

    let vram: Vec<ScreenColor> = gb.get_vram_data().into_iter().take(8).collect();
    assert_eq!(vram, colors(vec![0, 1, 2, 3, 0, 1, 2, 3]));
}

#[test]
fn setting_the_palette_color_sets_the_colors_of_the_sprites() {
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

    let vram_pixels: Vec<ScreenColor> = gb.get_vram_data().into_iter().take(4).collect();

    assert_eq!(vram_pixels, colors(vec![0, 3, 3, 3]));
}

#[test]
fn use_instruction_to_load_a_palette() {
    // A contains 0xE4
    // HL contains FF47
    // LD (HL), A
    let mut gb = Gameboy::new(vec![0x77]);
    gb.set_register_8(RegisterLabel8::A, DEFAULT_PALLETE);
    gb.set_register_16(RegisterLabel16::HL, Labels::BG_PALETTE);

    gb.step_once();

    // The palette should now be set

    gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x0, 0x50);
    gb.set_memory_at(Labels::CHARACTER_RAM_START + 0x1, 0x30);

    // This sprite is:
    // 0, 1, 2, 3
    // So the resulting pixels should be WHITE, LIGHT, DARK, BLACK

    let vram_pixels: Vec<ScreenColor> = gb.get_vram_data().into_iter().take(4).collect();

    assert_eq!(vram_pixels, colors(vec![0, 1, 2, 3]));
}

#[test]
fn the_ppu_should_set_the_ly_register_every_456_clocks_when_screen_on() {
    let mut gb = ppu_infinite_loop_gb();

    // Run an infinite loop
    for _ in 0..28 {
        // At every cycle check that the LY counter is 0
        gb.step_once();
        gb.step_once();
        assert_eq!(gb.get_memory_at(Labels::LCDC_Y), 0);
    }

    // Every 456 clocks the LY register will tick up
    gb.step_once();
    gb.step_once();

    assert_eq!(gb.get_memory_at(Labels::LCDC_Y), 1);

    // If we set the LY to 153 and tick up it will wrap around to 0
    gb.set_memory_at(Labels::LCDC_Y, 153);

    // Flip over the scan line
    render_line(&mut gb);

    assert_eq!(gb.get_memory_at(Labels::LCDC_Y), 0);

    // Turn off the screen and run over a scan line
    gb.set_memory_at(0xFF00, 0);
}

#[test]
fn turning_off_the_lcd_screen_resets_ly_register_to_0() {
    let mut gb = ppu_infinite_loop_gb();

    render_line(&mut gb);

    // set the screen to off
    gb.set_memory_at(Labels::V_BLANK, 0);

    // The LY register should be set to zero
    assert_eq!(gb.get_memory_at(Labels::LCDC_Y), 0);
}

#[test]
fn retrieving_the_screen_colors_gets_the_top_left_of_the_vram() {
    let mut gb = ppu_infinite_loop_gb();

    // Add a sprite to the top let corner of vram
    let sprite1 = [
        0x50, 0x30, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    add_sprite_to_vram(&mut gb, 1, &sprite1);
    gb.set_memory_at(Labels::BG_MAP_DATA_1_START, 1);

    let sprite2 = [
        0xA, 0xC, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    add_sprite_to_vram(&mut gb, 2, &sprite2);
    gb.set_memory_at(Labels::BG_MAP_DATA_1_START + 19, 2);

    // This sprite is:
    // 0, 1, 2, 3
    // So the resulting pixels should be WHITE, LIGHT, DARK, BLACK

    let screen_data_pre = gb.get_screen_data();
    assert_eq!(screen_data_pre.len(), 144 * 160);

    render_line(&mut gb);

    // The first line of the screen will have been drawn
    // starting with: 0, 1, 2, 3 & finishing with 3, 2, 1, 0
    let screen_data_post = gb.get_screen_data();
    let first_part: Vec<ScreenColor> = screen_data_post.iter().take(4).map(|x| *x).collect();
    let last_part: Vec<ScreenColor> = screen_data_post
        .iter()
        .take(160)
        .skip(156)
        .map(|x| *x)
        .collect();
    assert_eq!(first_part, colors(vec![0, 1, 2, 3]));
    assert_eq!(last_part, colors(vec![3, 2, 1, 0]));
}

#[test]
fn move_the_screen_position_will_move_what_is_displayed_on_screen() {
    let mut gb = ppu_infinite_loop_gb();

    // Move the screen down and right
    gb.set_memory_at(Labels::SCROLL_Y, 1);
    gb.set_memory_at(Labels::SCROLL_X, 1);

    let sprite = [
        0x0, 0x0, 0x50, 0x30, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    add_sprite_to_vram(&mut gb, 1, &sprite);
    gb.set_memory_at(Labels::BG_MAP_DATA_1_START, 1);

    render_line(&mut gb);

    // Get the screen data
    let screen_data = gb.get_screen_data();
    let first_part: Vec<ScreenColor> = screen_data.iter().take(4).map(|x| *x).collect();
    assert_eq!(first_part, colors(vec![1, 2, 3, 0]));
}

#[test]
fn screen_draws_correctly_when_it_wraps_around_the_side_and_bottom() {
    let mut gb = ppu_infinite_loop_gb();

    // Put a sprite in the top left corner of vram
    let sprite = vec![
        0x50, 0x30, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];
    add_sprite_to_vram(&mut gb, 1, &sprite);
    gb.set_memory_at(Labels::BG_MAP_DATA_1_START, 1);

    // Put the screen at the bottom-right corner of vram (255,255)
    gb.set_memory_at(Labels::SCROLL_X, 255);
    gb.set_memory_at(Labels::SCROLL_Y, 255);

    // Render the whole screen
    for _ in 0..144 {
        render_line(&mut gb);
    }

    let screen_data = gb.get_screen_data();

    print_screen_data(&screen_data);

    // The image will be at 0, 0 of the screen
    let mut first_pixels = vec![ScreenColor::White; 4];
    first_pixels.copy_from_slice(&screen_data[0..4]);

    assert_eq!(first_pixels, colors(vec![0, 1, 2, 3]));
}

fn render_line(gb: &mut Gameboy) {
    // Tick the gb for 456 clocks at which point the
    // first line of the screen will have been rendered
    // Loop through a line
    for _ in 0..29 {
        gb.step_once();
        gb.step_once();
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

fn add_sprite_to_vram(gb: &mut Gameboy, tile_index: u16, sprite_data: &[u8]) {
    for (index, val) in sprite_data.iter().enumerate() {
        gb.set_memory_at(
            Labels::CHARACTER_RAM_START + tile_index * 16 + index as u16,
            *val,
        );
    }
}

fn print_screen_data(screen_data: &[ScreenColor]) {
    let color_to_number = |col| match col {
        ScreenColor::White => 0,
        ScreenColor::Light => 1,
        ScreenColor::Dark => 2,
        _ => 3,
    };

    println!("[");
    screen_data.chunks(160).for_each(|colors| {
        let numbers: Vec<i32> = colors.iter().map(|color| color_to_number(*color)).collect();
        numbers.iter().for_each(|num| print!("{}", *num));
        println!("");
    });
    println!("]");
}
