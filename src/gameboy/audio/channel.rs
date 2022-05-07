use super::{
    timer::{TickResult, Timer},
    {get_duty, DutyCycle},
};

const CYCLES_PER_SECOND: i32 = 4194304;
const CYCLES_PER_PERIOD: i32 = CYCLES_PER_SECOND / 64;

pub struct Channel {
    channel_timer: Timer,
    period_timer: Timer, // 64Hz timer which reduces the period counter once hits 0. Actually the frame sequencer
    frequency: i32,      // Defines how quickly we move through the duty cycle
    volume: i32,         // The volume is reduced by 1 every time period counter hits 0
    duty: DutyCycle,
    duty_position: u8,
    enabled: bool,
}

impl Channel {
    pub fn new() -> Self {
        Self {
            channel_timer: Timer::new(),
            period_timer: Timer::new(),
            frequency: 0,
            volume: 0,
            duty: DutyCycle::Zero,
            duty_position: 0,
            enabled: false,
        }
    }

    pub fn tick(&mut self, dt: u32, memory: &mut [u8]) {
        // Set enabled from mem and trigger the channel
        if (memory[0xFF14] & 0b1000_0000) != 0 {
            self.trigger(memory);

            // Turn the channel on
            self.enabled = true;

            // Reset the trigger
            memory[0xFF14] &= 0b0111_1111;
        }

        // If enabled start counting the timers
        if self.enabled {
            // If the period timer has ticked reduce volume
            if self.period_timer.tick(dt) == TickResult::Ticked {
                self.volume -= 1;
                self.volume = self.volume.max(0);
            }

            // If the channel timer ticks increase the duty position
            if self.channel_timer.tick(dt) == TickResult::Ticked {
                self.duty_position = (self.duty_position + 1) % 8;
            }
        }

        if self.volume == 0 {
            // This doesn't seem to make a difference but means there is less
            // processing going on when no sound is playing.
            // Unable to unit test
            self.enabled = false;
        }
    }

    pub fn get_volume(&self) -> i16 {
        self.volume as i16 * get_duty(self.duty, self.duty_position) as i16
    }

    pub fn trigger(&mut self, memory: &[u8]) {
        self.frequency = Self::get_frequency(memory);
        self.volume = ((memory[0xFF12] & 0b1111_0000) >> 4) as i32;

        // Set period timer to max * counter + enable timer
        let period_counter = (memory[0xFF12] & 0b0000_0111) as i32;
        if period_counter != 0 {
            self.period_timer.start(CYCLES_PER_PERIOD * period_counter);
        }

        // Set the channel timer from the frequency
        self.channel_timer.start((2048 - self.frequency) * 4);

        // Set the duty
        self.duty = DutyCycle::from((memory[0xFF11] & 0b1100_0000) >> 6);
    }

    fn get_frequency(memory: &[u8]) -> i32 {
        let freq_lsb = (memory[0xFF13]) as i32;
        let freq_msb = (memory[0xFF14] & 0b0000_0111) as i32;
        (freq_msb << 8) | freq_lsb
    }
}
