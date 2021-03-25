#[derive(Clone, Copy)]
enum DutyCycle {
    Zero,
    One,
    Two,
    Three,
}

fn get_duty(duty: DutyCycle, pos: u8) -> u8 {
    let index = pos as usize;
    match duty {
        DutyCycle::Zero => [0, 0, 0, 0, 0, 0, 0, 1][index],
        DutyCycle::One => [1, 0, 0, 0, 0, 0, 0, 1][index],
        DutyCycle::Two => [0, 0, 0, 0, 0, 1, 1, 1][index],
        DutyCycle::Three => [0, 1, 1, 1, 1, 1, 1, 0][index],
    }
}

pub struct ALU<'a> {
    audio_callback: Box<dyn FnMut(i16) + 'a>,
    cycles_to_next_sample: i32,
    channel_timer: i32,
    period_timer: i32, // 64Hz timer which reduces the period counter once hits 0. Actually the frame sequencer
    period_counter: i32,
    frequency: i32, // Defines how quickly we move through the duty cycle
    volume: i32,    // The colume is reduced by 1 every time period counter hits 0
    duty: DutyCycle,
    duty_position: u8,
    enabled: bool,
}

const SAMPLE_RATE: i32 = 44100; // Hz
const CYCLES_PER_SECOND: i32 = 4194304;
const CYCLES_PER_SAMPLE: i32 = CYCLES_PER_SECOND / SAMPLE_RATE;
const CYCLES_PER_PERIOD: i32 = CYCLES_PER_SECOND / 64;
const CYCLES_PER_LENGTH: i32 = CYCLES_PER_SECOND / 256;

impl<'a> ALU<'a> {
    pub fn new<F>(audio_callback: F) -> ALU<'a>
    where
        F: FnMut(i16) + 'a,
    {
        ALU {
            audio_callback: Box::new(audio_callback),
            cycles_to_next_sample: CYCLES_PER_SAMPLE,
            channel_timer: 0,
            period_timer: 0,
            period_counter: 0,
            frequency: 0,
            volume: 0,
            enabled: false,
            duty: DutyCycle::One,
            duty_position: 0,
        }
    }

    pub fn tick(&mut self, tick: u32, memory: &mut Vec<u8>) {
        // decrease the cycles_to_next_sample
        self.cycles_to_next_sample = self.cycles_to_next_sample - tick as i32;

        // Set enabled from mem and do other stuff (if enabled already false)
        if (memory[0xFF14] & 0b1000_0000) != 0 {
            // Set the frequency from mem
            let freq_lsb = (memory[0xFF13]) as i32;
            let freq_msb = (memory[0xFF14] & 0b0000_0111) as i32;
            let frequency = (freq_msb << 8) | freq_lsb;

            // Get the volume
            let volume = (memory[0xFF12] & 0b1111_0000) >> 4;

            if frequency > 0 && volume > 0 {
                self.frequency = frequency;
                self.volume = volume as i32;

                // Set period timer to max
                self.period_timer = CYCLES_PER_PERIOD;

                // Set the period counter
                self.period_timer = (memory[0xFF12] & 0b0000_0111) as i32;

                // Set the channel timer from the frequency
                self.channel_timer = (2048 - self.frequency) * 4;

                // Set the duty
                self.duty = match (memory[0xFF11] & 0b1100_0000) >> 6 {
                    0 => DutyCycle::Zero,
                    1 => DutyCycle::One,
                    2 => DutyCycle::Two,
                    3 => DutyCycle::Three,
                    other => {
                        panic!("Could not set duty cycle from value {}", other)
                    }
                };

                // TODO: Set length bits

                // Turn the channel on
                self.enabled = true;

                // Reset the trigger
                memory[0xFF14] = memory[0xFF14] & 0b0111_1111;
            }
        }

        // If enabled start counting the timers
        if self.enabled {
            // Reduce the channel timer
            self.channel_timer -= tick as i32;

            // reduce the period timer
            self.period_timer -= tick as i32;

            // If the period timer is 0
            if self.period_timer <= 0 {
                // Reduce the period counter
                self.period_counter -= 1;

                // If the period counter is 0 reduce the volume
                if self.period_counter <= 0 {
                    self.volume -= 1;
                    self.volume = if self.volume <= 0 { 0 } else { self.volume };
                    self.period_counter = (memory[0xFF12] & 0b0000_0111) as i32;
                }

                // Reset the period counter
                self.period_timer = CYCLES_PER_PERIOD + self.period_timer;
            }

            // If the channel timer is 0 increase the duty position
            if self.channel_timer <= 0 {
                self.duty_position = (self.duty_position + 1) % 8;

                self.channel_timer = (2048 - self.frequency) * 4;
            }

            if self.volume == 0 {
                self.enabled = false;
            }
        }

        // if the cycles are less than 0 then emit a value, reset the count
        if self.cycles_to_next_sample <= 0 {
            let volume = self.volume as i16 * get_duty(self.duty, self.duty_position) as i16;
            self.audio_callback.as_mut()(volume);
            self.cycles_to_next_sample = CYCLES_PER_SAMPLE + self.cycles_to_next_sample;
        }
    }
}
