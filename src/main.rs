#![allow(clippy::comparison_chain)]

extern crate image as img;

#[macro_use]
extern crate lazy_static;

mod debug_cli;
mod gameboy;

use crate::debug_cli::{update, DebugControls, OpcodeWriter};
use crate::gameboy::{Gameboy, ScreenColor, TickResult};
use clap::{value_parser, Arg, ArgAction};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleRate, StreamConfig};
use fs::File;
use piston_window::*;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

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
const WINDOW_SCALING: u32 = 4;

#[derive(PartialEq)]
enum AppResult {
    Continue,
    Finish,
}

type OpcodeCallback<'a> = Box<dyn FnMut(u16, String) -> () + 'a>;

pub struct App<'a> {
    texture_context: G2dTextureContext,
    gb: Gameboy<'a>,
    is_debug: bool,
    breakpoints: Vec<u16>,
    opcode_writer: Option<OpcodeCallback<'a>>,
}

impl<'a> App<'a> {
    fn render(&mut self, c: Context, g: &mut piston_window::G2d) {
        let mut buffer: Vec<u8> = vec![0x00; (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize];

        // Put the screen data into the buffer
        self.gb
            .get_screen_data()
            .iter()
            .map(|col| screen_color_to_color(*col))
            .enumerate()
            .for_each(|(i, val)| {
                let scaled = i * 4;
                buffer[scaled] = val[0];
                buffer[scaled + 1] = val[1];
                buffer[scaled + 2] = val[2];
                buffer[scaled + 3] = val[3];
            });

        let canvas = img::ImageBuffer::from_vec(SCREEN_WIDTH, SCREEN_HEIGHT, buffer).unwrap();

        let mut texture_settings = TextureSettings::new();
        texture_settings.set_filter(Filter::Nearest);

        // Transform into a texture so piston can use it.
        let texture: G2dTexture =
            Texture::from_image(&mut self.texture_context, &canvas, &texture_settings).unwrap();

        piston_window::image(
            &texture,
            c.transform
                .scale(WINDOW_SCALING as f64, WINDOW_SCALING as f64),
            g,
        );
    }

    fn update(&mut self, args: UpdateArgs) -> AppResult {
        if self.is_debug {
            let debug_controls = update(&self.gb, &mut self.breakpoints);

            if let DebugControls::Continue = debug_controls {
                self.is_debug = false;
            }
        }

        if self.is_debug {
            self.gb.step_once();
        } else {
            let stop_reason =
                self.gb
                    .tick_with_breaks(args.dt, &self.breakpoints, &mut self.opcode_writer);

            match stop_reason {
                TickResult::HitBreakpoint => {
                    self.is_debug = true;
                }
                TickResult::Crash => {
                    return AppResult::Finish;
                }
                _ => {}
            }
        }
        return AppResult::Continue;
    }
}

fn build_audio_event_loop() -> impl DeviceTrait {
    // Create an audio device & event loop
    let host = cpal::default_host();
    host.default_output_device().unwrap()
}

fn create_audio_thread<T>(device: T, receiver: Receiver<i16>) -> impl StreamTrait
where
    T: DeviceTrait + Send + Sync + 'static,
{
    let my_config = StreamConfig {
        channels: 1,
        buffer_size: cpal::BufferSize::Default,
        sample_rate: SampleRate(44100),
    };
    device
        .build_output_stream(
            &my_config,
            move |data, _| {
                for elem in data.iter_mut() {
                    // Keep pulling values until no more are left. Then add 0s
                    match receiver.recv() {
                        Ok(data) => {
                            *elem = data as f32 / 100.0;
                        }
                        Err(_) => {
                            *elem = 0.0;
                        }
                    }
                }
            },
            move |_err| {},
            None,
        )
        .unwrap()
}

fn load_rom(file_name: &str) -> std::io::Result<Vec<u8>> {
    let mut rom_file = File::open(file_name)?;
    let mut rom_data = Vec::new();
    rom_file.read_to_end(&mut rom_data)?;
    Ok(rom_data)
}

fn main() {
    let gb_screen_height = SCREEN_HEIGHT;
    let gb_screen_width = SCREEN_WIDTH;

    let matches = clap::Command::new("RustGB")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Start in debug mode")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("log")
                .short('l')
                .long("log")
                .help("Record a logfile of all instructions")
                .action(ArgAction::Set)
                .value_name("FILE")
                .value_parser(value_parser!(PathBuf))
                .required(false),
        )
        .arg(Arg::new("ROM").required(true).help("Start with rom"))
        .get_matches();

    // Create a channel which takes audio data
    let (sender, receiver) = channel::<i16>();

    let audio_callback = move |val| match sender.send(val) {
        Ok(_) => {}
        Err(err) => {
            println!("Error occurred {}", err);
            panic!("Something went wrong");
        }
    };

    let mut window: PistonWindow = WindowSettings::new(
        "Gameboy",
        [
            gb_screen_width * WINDOW_SCALING,
            gb_screen_height * WINDOW_SCALING,
        ],
    )
    .exit_on_esc(true)
    .resizable(false)
    .build()
    .unwrap();

    let is_debug = matches.get_flag("debug");
    let mut opcode_writer = matches
        .get_one::<PathBuf>("log")
        .map(|path| OpcodeWriter::new(path));

    // Load the ROM
    let rom_file_name = matches.get_one::<String>("ROM").unwrap();
    let rom_data = load_rom(rom_file_name);

    let rom_bytes = match rom_data {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to load ROM with error {}", err);
            return;
        }
    };

    {
        let writer: Option<Box<dyn FnMut(u16, String)>> = opcode_writer.as_mut().map(|w| {
            Box::new(|address, opcode| w.store_opcode(address, opcode))
                as Box<dyn FnMut(u16, String)>
        });

        let mut app = App {
            texture_context: window.create_texture_context(),
            gb: Gameboy::new_with_bootloader(audio_callback, &rom_bytes),
            is_debug,
            breakpoints: vec![],
            opcode_writer: writer,
        };

        let stream;
        if !is_debug {
            let device = build_audio_event_loop();
            stream = create_audio_thread(device, receiver);
            stream.play().unwrap();
        }

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(_r) = e.render_args() {
                window.draw_2d(&e, |context, g, _| {
                    app.render(context, g);
                });
            }

            if let Some(u) = e.update_args() {
                if app.update(u) == AppResult::Finish {
                    break;
                }
            }
        }
    }

    // Write the log
    if let Some(mut writer) = opcode_writer {
        writer.write_file();
    }
}
