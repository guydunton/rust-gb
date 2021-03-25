use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use piston_window::*;
use std::sync::mpsc::channel;
use std::thread;
use std::{env, sync::mpsc::Receiver};
use thread::JoinHandle;

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

pub struct App<'a> {
    texture_context: G2dTextureContext,
    gb: Gameboy<'a>,
    is_debug: bool,
    breakpoints: Vec<u16>,
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

        // Transform into a texture so piston can use it.
        let texture: G2dTexture =
            Texture::from_image(&mut self.texture_context, &canvas, &TextureSettings::new())
                .unwrap();

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

fn build_audio_event_loop() -> impl EventLoopTrait {
    // Create an audio device & event loop
    let host = cpal::default_host();
    let event_loop = host.event_loop();
    let device = host.default_output_device().unwrap();
    let format_base = device.default_output_format().unwrap();
    let format = cpal::Format {
        channels: 2,
        sample_rate: format_base.sample_rate,
        data_type: cpal::SampleFormat::F32,
    };

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop
        .play_stream(stream_id)
        .expect("failed to play_stream");

    event_loop
}

fn create_audio_thread<T>(event_loop: T, receiver: Receiver<i16>) -> JoinHandle<()>
where
    T: EventLoopTrait + Send + Sync + 'static,
{
    // Create thread containing channel receiver and event loop.
    thread::spawn(move || {
        // Thread runs the event loop which pulls from the channel
        event_loop.run(move |stream_id2, stream_result| {
            let mut successes = 0;
            let mut failures = 0;

            let stream_data = match stream_result {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("an error occurred on stream {:?}: {}", stream_id2, err);
                    return;
                }
            };

            use cpal::{StreamData, UnknownTypeOutputBuffer};
            match stream_data {
                StreamData::Output {
                    buffer: UnknownTypeOutputBuffer::F32(mut buffer),
                } => {
                    for elem in buffer.iter_mut() {
                        // Keep pulling values until no more are left. Then add 0s
                        match receiver.recv() {
                            Ok(data) => {
                                *elem = data as f32 / 100.0;
                                successes += 1;
                            }
                            Err(_) => {
                                *elem = 0.0;
                                failures += 1
                            }
                        }
                    }
                }
                _ => (),
            }

            let total = successes + failures;
            println!("{}/{}", successes / total * 100, failures / total * 100);
        });
    })
}

fn main() {
    let gb_screen_height = SCREEN_HEIGHT;
    let gb_screen_width = SCREEN_WIDTH;

    // Create a channel which takes audio data
    let (sender, receiver) = channel::<i16>();

    let audio_callback = move |val| {
        sender.send(val).unwrap();
    };

    let mut window: PistonWindow =
        WindowSettings::new("Gameboy", [gb_screen_width * 2, gb_screen_height * 2])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let args: Vec<String> = env::args().collect();

    let is_debug = args.contains(&String::from("-d"));

    let mut app = App {
        texture_context: window.create_texture_context(),
        gb: Gameboy::new_with_bootloader(audio_callback),
        is_debug,
        breakpoints: vec![],
    };

    let _join_handle: JoinHandle<()>;

    if !is_debug {
        let event_loop = build_audio_event_loop();
        _join_handle = create_audio_thread(event_loop, receiver);
    }

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
