use std::sync::{Arc, Mutex};
use std::time::Duration;

use rodio::source::Source;
use rodio::{cpal, DeviceTrait, OutputStream, Sink};

use emulator::{Emulator, Memory};

const WAVE_PATTERNS: [u8; 4] = [0x80, 0xC0, 0xF0, 0xFC];

// Why do all my projects end up being poorly organized?
const CYCLES_PER_SECOND: u64 = 4194304; // where did this magic number come from?

// Quadrangular (i.e. Square) Wave
#[derive(Debug)]
struct QuadWave {
    has_sweep: bool,
    base_addr: u16,
    phase: f32,
    time_since_sweep: f32,

    emu: Arc<Mutex<Emulator>>,
    last_tick: u64,
}

impl QuadWave {
    fn new(emu: Arc<Mutex<Emulator>>, has_sweep: bool, base_addr: u16) -> QuadWave {
        QuadWave {
            has_sweep,
            base_addr,
            phase: 0.0,
            time_since_sweep: 0.0,
            emu,
            last_tick: 0,
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
    fn read_frequency(&self, mem: &Memory) -> (u32, bool, bool) {
        let data = mem.rw(self.base_addr + 2) as u32;
        let x = data & 0x07FF;
        (131072 / (2048 - x), data & 0x4000 > 0, data & 0x8000 > 0)
    }
    fn write_frequency(&self, mem: &mut Memory, freq: u16) {
        let mut data = mem.rw(self.base_addr + 2);
        data = (data & 0xF800) | (freq & 0x07FF);
        mem.ww(self.base_addr + 2, data);
    }
    fn sample(&mut self, ds: f32) -> f32 {
        let mut emu = self.emu.lock().unwrap();
        let (duty, _) = self.read_len_wave_duty(&emu.mem);
        let down_time = match duty {
            0 => 0.125,
            1 => 0.25,
            2 => 0.5,
            _ => 0.75,
        };
        let (freq, _, _) = self.read_frequency(&emu.mem);
        let cycle_length = 1.0 / (freq as f32);

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
            1.0
        }
    }
}

impl Iterator for QuadWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        println!("next called");
        let tick = self.emu.lock().unwrap().get_clock();
        let ds = (tick - self.last_tick) as f32 / CYCLES_PER_SECOND as f32;
        let sample = self.sample(ds);
        self.last_tick = tick;
        println!("sample: {}", sample);
        Some(sample)
    }
}

impl Source for QuadWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        let emu = self.emu.lock().unwrap();
        self.read_frequency(&emu.mem).0
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct SoundManager {
    // Need to keep this alive for sound to keep going
    stream: OutputStream,
    sinks: [Sink; 4],
    paused: bool,
}

impl SoundManager {
    pub fn new(emu: Arc<Mutex<Emulator>>) -> Result<SoundManager, rodio::PlayError> {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        let sinks: [Sink; 4] = [
            Sink::try_new(&stream_handle)?,
            Sink::try_new(&stream_handle)?,
            Sink::try_new(&stream_handle)?,
            Sink::try_new(&stream_handle)?,
        ];
        for sink in &sinks {
            sink.pause();
        }
        sinks[0].append(QuadWave::new(emu.clone(), true, 0xFF11));
        sinks[1].append(QuadWave::new(emu.clone(), false, 0xFF16));

        Ok(SoundManager {
            stream,
            sinks,
            paused: true,
        })
    }
    pub fn play(&mut self) {
        self.paused = false;
        for sink in &self.sinks {
            sink.play();
        }
    }
    pub fn pause(&mut self) {
        self.paused = true;
        for sink in &self.sinks {
            sink.pause();
        }
    }
    pub fn toggle_paused(&mut self) {
        if self.paused {
            self.play();
        } else {
            self.pause();
        }
    }
}