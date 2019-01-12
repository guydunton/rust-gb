use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{event_loop::*, input::*, window::WindowSettings};
use rand::prelude::*;
use std::env;

mod gameboy;
use crate::gameboy::{screen::*, Gameboy};

use std::io;

fn screen_color_to_color(c: ScreenColor) -> [f32; 4] {
    match c {
        ScreenColor::Black => [0.1, 0.2, 0.1, 1.0],
        ScreenColor::Dark => [0.2, 0.4, 0.2, 1.0],
        ScreenColor::Light => [0.5, 0.7, 0.1, 1.0],
        ScreenColor::White => [0.6, 0.7, 0.1, 1.0],
    }
}

fn render_screen(screen: &Screen, c: &graphics::Context, gl: &mut GlGraphics) {
    use graphics::*;
    let square = rectangle::square(0.0, 0.0, 2.0);

    for row in 0..144 {
        for col in 0..160 {
            let pixel = screen.pixels()[col + row * 160];
            let color = screen_color_to_color(pixel);
            let (x, y) = (col * 2, row * 2);

            let transform = c.transform.trans(x as f64, y as f64);
            rectangle(color, square, transform, gl);
        }
    }
}

pub struct App {
    gl: GlGraphics,
    screen: Screen,
    gb: Gameboy,
    is_debug: bool,
}

fn print_help() {
    println!("c => continue");
    println!("r => print registers");
    println!("o => print current opcode");
    println!("f => print flags");
    println!("p => print opcodes");
    println!("h => help");
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let screen = &self.screen;

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(WHITE, gl);
            render_screen(&screen, &c, gl);
        });
    }

    fn update(&mut self, args: UpdateArgs, rng: &mut ThreadRng) {
        if self.is_debug {
            loop {
                println!("Continue? (h for help)");
                let mut text = String::new();
                io::stdin()
                    .read_line(&mut text)
                    .expect("Input failed unexpectadly");
                let trimmed = text.trim();
                match trimmed.as_ref() {
                    "c" => break,
                    "r" => self.print_registers(),
                    "o" => println!("{}", self.gb.print_opcode()),
                    "f" => println!("{:?}", self.gb.print_flags()),
                    "h" => print_help(),
                    "p" => self.print_opcodes(),
                    _ => print_help(),
                }
            }
        }

        if self.is_debug {
            self.gb.step_once();
        } else {
            self.gb.tick(args.dt);
        }

        let mut pixels = Vec::new();
        for _ in 0..160 {
            let num = rng.gen_range(0, 4);
            let px_color = match num {
                0 => ScreenColor::White,
                1 => ScreenColor::Light,
                2 => ScreenColor::Dark,
                _ => ScreenColor::Black,
            };
            pixels.push(px_color);
        }

        self.screen.push_pixels(&pixels);
    }

    fn print_registers(&self) {
        let registers = self.gb.get_registers();
        let register_order = vec![
            "A", "F", "AF", "B", "C", "BC", "D", "E", "DE", "H", "L", "HL", "PC", "SP",
        ];
        for register in register_order {
            println!(
                "{}: {}",
                register,
                registers.get_register_val(&register.to_string())
            );
        }
    }

    fn print_opcodes(&self) {
        let instructions = self.gb.print_instructions();

        println!("-------------------------------------------");
        println!("{:<width$} : {}", "Address", "Opcode", width = 10);
        println!("-------------------------------------------");
        for instruction in instructions {
            println!(
                "{:<#width$X} : {}",
                instruction.get_address(),
                instruction.get_opcode(),
                width = 10
            );
        }
        println!("-------------------------------------------");
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Gameboy", [320, 288])
        .opengl(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let args: Vec<String> = env::args().collect();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        screen: Screen::new(),
        gb: Gameboy::new(),
        is_debug: args.contains(&String::from("-d")),
    };

    let mut rnd = rand::thread_rng();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(u, &mut rnd);
        }
    }
}
