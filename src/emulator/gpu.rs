use emulator::memory::Memory;

const SCANLINE_TOTAL_TIME: i16 = 456;
const SCANLINE_MODE2_OVER: i16 = 456-80;
const SCANLINE_MODE3_OVER: i16 = 456-80-172;

#[allow(dead_code)]
pub struct Gpu {
	//scanline counter
	sl_count:		i16,
	//Scroll Registers
	scx: 			u8,
	scy: 			u8,
	//Background Palette Register
	bp:				u8,
}

impl Gpu {
	pub fn new() -> Gpu {
	    Gpu{sl_count: 0, scx: 0, scy: 0, bp: 0}
	}
	pub fn step(&mut self, mem: &mut Memory, cycles: i16) {
		self.set_lcd_status(mem);
		if self.is_lcd_enabled(mem) {
			self.sl_count -= cycles;
			if self.sl_count <= 0 {
				let line = (mem.rb(0xFF44) + 1)%154;
				mem.wb(0xFF44, line);

				self.sl_count = SCANLINE_TOTAL_TIME;
				if line == 144 {
					//Request Interupt
				} else if line < 144 {
					//Draw the line
				}
			}
		}
	}
	fn set_lcd_status(&mut self, mem: &mut Memory) {
		let mut status = mem.rb(0xFF41);
		let line = mem.rb(0xFF44);
		let mode = status & 3;

		let mut request_interrupt = false;
		if !self.is_lcd_enabled(mem) {
			self.sl_count = SCANLINE_TOTAL_TIME;
			mem.wb(0xFF44, 0);
			status = (status & 0xFC) | 1;
		} else if line > 144 {
			status = (status & 0xFC) | 1;
			request_interrupt = (status & (1 << 4)) > 0;
		} else if self.sl_count >= SCANLINE_MODE2_OVER {
			status = (status & 0xFC) | 2;
			request_interrupt = (status & (1 << 5)) > 0;
		} else if self.sl_count >= SCANLINE_MODE3_OVER {
			status = (status & 0xFC) | 3;
		} else {
			status = status & 0xFC;
			request_interrupt = (status & (1 << 3)) > 0;
		}

		if request_interrupt && (mode != (status & 3)) {
			//Request Interupt
		}
		if line == mem.rb(0xFF45) && self.is_lcd_enabled(mem) {
			status = (status & 0xFB) | 4;
			if (status & (1 << 6)) > 0 {
				//Request Interupt
			} else {
				status = status & 0xFB;
			}
		}

		mem.wb(0xFF41, status);
	}
	fn is_lcd_enabled(&self, mem: &Memory) -> bool {
		(mem.rb(0xFF40) & (1 << 7)) > 0
	}
}