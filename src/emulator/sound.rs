use std::sync::Arc;

use sdl2::audio::AudioCallback;

use emulator::Memory;

const WAVE_PATTERNS: [u8; 4] = [0x80, 0xC0, 0xF0, 0xFC];

// Quadrangular (i.e. Square) Wave
struct QuadWave {
	mem: Arc<Memory>,
	has_sweep: bool,
	base_addr: u16,
	phase: f32,
}

impl QuadWave {
	fn new(mem: Arc<Memory>, has_sweep: bool, base_addr: u16) -> QuadWave {
		QuadWave {
			mem: mem,
			has_sweep: has_sweep,
			base_addr: base_addr,
			phase: 0.0,
		}
	}
	// Returns (sweep time, sweep +/-, number of sweep shift)
	fn read_sweep_reg(&self) -> (u8, bool, u8) {
		let data = self.mem.rb(self.base_addr-1);
		((data & 0x70) >> 4, (data & 0x08) == 0, data & 0x07)
	}
	// returns (wave duty, sound length data)
	fn read_len_wave_duty(&self) -> (u8, u8) {
		let data = self.mem.rb(self.base_addr);
		(WAVE_PATTERNS[data as usize >> 6], data & 0x3F)
	}
	// returns (initial volume, direction, number of envelope sweep)
	fn read_volume_envelope(&self) -> (u8, bool, u8) {
		let data = self.mem.rb(self.base_addr+1);
		((data & 0xF0) >> 4, data & 0x08 > 0, data & 0x07)
	}
	// returns (frequency, counter/consecutive selection, initial)
	fn read_frequency(&self) -> (u16, bool, bool) {
		let data = self.mem.rw(self.base_addr+2);
		(data & 0x07FF, data & 0x4000 > 0, data & 0x8000 > 0)
	}
	fn sample(&mut self) -> f32 {
		let (duty, _) = self.read_len_wave_duty();
		let down_time = match duty {
			0 => 0.125, 1 => 0.25, 2 => 0.5, _ => 0.75
		};
		let (freq, _, _) = self.read_frequency();
		let freq_hz = 131072.0/(2048 - freq) as f32;
		
		if self.phase < down_time {
			0.0
		} else {
			1.0
		}
	}
}

pub struct SoundManager {
	//mem: Arc<Memory>,
	sound1: QuadWave,
	sound2: QuadWave,
	phase_inc: f32,
}

impl SoundManager {
	pub fn new(mem: Arc<Memory>, freq: i32) -> SoundManager {
		SoundManager {
			sound1: QuadWave::new(mem.clone(), true, 0xFF11),
			sound2: QuadWave::new(mem.clone(), false, 0xFF16),
			phase_inc: 440.0 / freq as f32,
		}
	}
}

impl AudioCallback for SoundManager {
	type Channel = f32;

	fn callback(&mut self, out: &mut [Self::Channel]) {
		for x in out.iter_mut() {
			*x = self.sound1.sample();
			self.sound1.phase = (self.sound1.phase + self.phase_inc)%1.0;
		}
	}
}
