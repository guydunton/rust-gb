use piston_window::*;
use std::env;

extern crate image as img;

#[macro_use]
extern crate lazy_static;

mod debug_cli;
mod gameboy;
use crate::debug_cli::{update, DebugControls};
use crate::gameboy::{Gameboy, ScreenColor, TickResult};

fn screen_color_to_color(c: ScreenColor) -> [u8; 4] {
    match c {
        ScreenColor::Black => [26, 51, 26, 255],
        ScreenColor::Dark => [51, 102, 51, 255],
        ScreenColor::Light => [128, 179, 26, 255],
        ScreenColor::White => [153, 179, 26, 255],
    }
}

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;


pub struct App {
    texture_context: G2dTextureContext,
    gb: Gameboy,
    is_debug: bool,
    breakpoints: Vec<u16>,
}

impl App {

    fn render(&mut self, c: Context, g: &mut piston_window::G2d) {
        let mut buffer: Vec<u8> = vec![0x00; (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize];

        // Put the screen data into the buffer
        self.gb.get_screen_data()
            .iter()
            .map(|col| screen_color_to_color(*col))
            .enumerate()
            .for_each(|(i, val)| {
                let scaled = i * 4;
                buffer[scaled] = val[0];
                buffer[scaled+1] = val[1];
                buffer[scaled+2] = val[2];
                buffer[scaled+3] = val[3];
            });

        let canvas = 
            img::ImageBuffer::from_vec(
                SCREEN_WIDTH, 
                SCREEN_HEIGHT,
                buffer
            ).unwrap();
    
        // Transform into a texture so piston can use it.
        let texture: G2dTexture = Texture::from_image(
            &mut self.texture_context,
            &canvas,
            &TextureSettings::new()
        ).unwrap();

        piston_window::image(&texture, c.transform.scale(2.0, 2.0), g);
    }

    fn update(&mut self, args: UpdateArgs) {
        if self.is_debug {
            let debug_controls = update(&self.gb, &mut self.breakpoints);

            match debug_controls {
                DebugControls::Continue => {
                    self.is_debug = false;
                }
                _ => {
                    // Do nothing
                }
            }
        }

        if self.is_debug {
            self.gb.step_once();
        } else {
            let stop_reason = self.gb.tick(args.dt, &self.breakpoints);

            if stop_reason == TickResult::HitBreakpoint {
                self.is_debug = true;
            }
        }
    }
}

fn main() {

    let gb_screen_height = SCREEN_HEIGHT;
    let gb_screen_width = SCREEN_WIDTH;

    //let gb_screen_height = 256;
    //let gb_screen_width = 256;

    let mut window: PistonWindow =
        WindowSettings::new("Gameboy", [gb_screen_width * 2, gb_screen_height * 2])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let args: Vec<String> = env::args().collect();

    let mut app = App {
        texture_context:window.create_texture_context(),
        gb: Gameboy::new_with_bootloader(),
        is_debug: args.contains(&String::from("-d")),
        breakpoints: vec![],
    };


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |context, g, _| {
                app.render(context, g);
            });
        }

        if let Some(u) = e.update_args() {
            app.update(u);
        }
    }
}
