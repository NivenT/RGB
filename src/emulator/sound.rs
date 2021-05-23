use std::sync::{Arc, Mutex};
use std::time::Duration;

//use sdl2::audio::AudioCallback;
use rodio::source::Source;

use emulator::{Emulator, Memory};

const WAVE_PATTERNS: [u8; 4] = [0x80, 0xC0, 0xF0, 0xFC];
const VOLUME: f32 = 0.25;

// Why do all my projects end up being poorly organized?
const CYCLES_PER_SECOND: u64 = 4194304; // where did this magic number come from?

// Quadrangular (i.e. Square) Wave
#[derive(Debug, Clone)]
struct QuadWave {
    has_sweep: bool,
    base_addr: u16,
    phase: f32,
    time_since_sweep: f32,
}

impl QuadWave {
    fn new(has_sweep: bool, base_addr: u16) -> QuadWave {
        QuadWave {
            has_sweep: has_sweep,
            base_addr: base_addr,
            phase: 0.0,
            time_since_sweep: 0.0,
        }
    }
    // Returns (sweep time, sweep +/-, number of sweep shift)
    fn read_sweep_reg(&self, mem: &Memory) -> (u8, bool, u8) {
        let data = mem.rb(self.base_addr - 1);
        ((data & 0x70) >> 4, (data & 0x08) == 0, data & 0x07)
    }
    // returns (wave duty, sound length data)
    fn read_len_wave_duty(&self, mem: &Memory) -> (u8, u8) {
        let data = mem.rb(self.base_addr);
        (WAVE_PATTERNS[data as usize >> 6], data & 0x3F)
    }
    // returns (initial volume, direction, number of envelope sweep)
    fn read_volume_envelope(&self, mem: &Memory) -> (u8, bool, u8) {
        let data = mem.rb(self.base_addr + 1);
        ((data & 0xF0) >> 4, data & 0x08 > 0, data & 0x07)
    }
    // returns (frequency, counter/consecutive selection, initial)
    fn read_frequency(&self, mem: &Memory) -> (u16, bool, bool) {
        let data = mem.rw(self.base_addr + 2);
        (data & 0x07FF, data & 0x4000 > 0, data & 0x8000 > 0)
    }
    fn write_frequency(&self, mem: &mut Memory, freq: u16) {
        let mut data = mem.rw(self.base_addr + 2);
        data = (data & 0xF800) | (freq & 0x07FF);
        mem.ww(self.base_addr + 2, data);
    }
    fn sample(&mut self, emu: &mut Emulator, ds: f32) -> f32 {
        let (duty, _) = self.read_len_wave_duty(&emu.mem);
        let down_time = match duty {
            0 => 0.125,
            1 => 0.25,
            2 => 0.5,
            _ => 0.75,
        };
        let (freq, _, _) = self.read_frequency(&emu.mem);
        let freq_hz = 131072.0 / (2048 - freq) as f32;
        let cycle_length = 1.0 / freq_hz;

        self.time_since_sweep += ds;
        let (sweep_time, sweep_pm, sweep_num) = self.read_sweep_reg(&emu.mem);
        let sweep_time_s = sweep_time as f32 / 128.0;
        if self.time_since_sweep > sweep_time_s {
            let scale = if sweep_pm { 1.0 } else { -1.0 };
            let new_freq_hz = (freq as f32) * (1.0 + scale / 2f32.powi(sweep_num as i32));
            let new_freq = (131072.0 - new_freq_hz * 2048.0) / new_freq_hz;

            self.write_frequency(&mut emu.mem, new_freq as u16);
            self.time_since_sweep = 0.0;
        }

        self.phase = (self.phase + ds / cycle_length) % 1.0;
        if self.phase < down_time {
            0.0
        } else {
            VOLUME
        }
    }
}

#[derive(Debug, Clone)]
pub struct SoundManager {
    emu: Arc<Mutex<Emulator>>,
    sound1: QuadWave,
    sound2: QuadWave,
    last_tick: u64,
}

impl SoundManager {
    pub fn new(emu: Arc<Mutex<Emulator>>) -> SoundManager {
        SoundManager {
            emu,
            sound1: QuadWave::new(true, 0xFF11),
            sound2: QuadWave::new(false, 0xFF16),
            last_tick: 0,
        }
    }
}

impl Iterator for SoundManager {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut emu = self.emu.lock().unwrap();
        let tick = emu.get_clock();
        let ds = (tick - self.last_tick) as f32 / CYCLES_PER_SECOND as f32;
        Some(self.sound1.sample(&mut emu, ds))
    }
}

impl Source for SoundManager {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        2 // should be 4
    }
    fn sample_rate(&self) -> u32 {
        441000 // should actually be something else. Fix things later
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
