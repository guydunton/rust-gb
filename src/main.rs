use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{event_loop::*, input::*, window::WindowSettings};
use std::env;

mod debug_cli;
mod gameboy;
use crate::debug_cli::update;
use crate::gameboy::{screen::*, Gameboy, RegisterLabel16};

fn screen_color_to_color(c: ScreenColor) -> [f32; 4] {
    match c {
        ScreenColor::Black => [0.1, 0.2, 0.1, 1.0],
        ScreenColor::Dark => [0.2, 0.4, 0.2, 1.0],
        ScreenColor::Light => [0.5, 0.7, 0.1, 1.0],
        ScreenColor::White => [0.6, 0.7, 0.1, 1.0],
    }
}

fn render_vram(gb: &Gameboy, c: &graphics::Context, gl: &mut GlGraphics) {
    use graphics::*;
    let square = rectangle::square(0.0, 0.0, 2.0);

    let vram_pixels = gb.get_vram_data();

    for row in 0..256 {
        for col in 0..256 {
            let pixel = vram_pixels[col + row * 256];
            let color = screen_color_to_color(pixel);
            let (x, y) = (col * 2, row * 2);

            let transform = c.transform.trans(x as f64, y as f64);
            rectangle(color, square, transform, gl);
        }
    }
}

pub struct App {
    gl: GlGraphics,
    gb: Gameboy,
    is_debug: bool,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let gb = &self.gb;

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(WHITE, gl);
            render_vram(&gb, &c, gl);
        });
    }

    fn update(&mut self, args: UpdateArgs) {
        if self.is_debug {
            update(&self.gb);
        }

        let breakpoint = 0xFFFF;

        if self.is_debug {
            self.gb.step_once();
        } else {
            self.gb.tick(args.dt, breakpoint);
            if self.gb.get_register_16(RegisterLabel16::ProgramCounter) == breakpoint {
                self.is_debug = true;
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    //let gb_screen_height = 144;
    //let gb_screen_width = 160;

    let gb_screen_height = 256;
    let gb_screen_width = 256;

    let mut window: Window =
        WindowSettings::new("Gameboy", [gb_screen_width * 2, gb_screen_height * 2])
            .opengl(opengl)
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let args: Vec<String> = env::args().collect();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        gb: Gameboy::new_with_bootloader(),
        is_debug: args.contains(&String::from("-d")),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(u);
        }
    }
}
