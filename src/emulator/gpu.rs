use emulator::memory::Memory;

const SCANLINE_TOTAL_TIME: i16 = 456;
const SCANLINE_MODE2_OVER: i16 = 456-80;
const SCANLINE_MODE3_OVER: i16 = 456-80-172;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Color{WHITE = 255, LIGHT_GRAY = 192, DARK_GRAY = 96, BLACK = 0}

impl Color {
	fn from_palette(id: u8, palette: u8) -> Color {
		let (hi, lo) = (2*id+1, 2*id);
		let color = ((palette & (1 << hi)) >> (hi-1)) | ((palette & (1 << lo)) >> lo);
		match color {
			0 => Color::WHITE,
			1 => Color::LIGHT_GRAY,
			2 => Color::DARK_GRAY,
			3 => Color::BLACK,
			_ => panic!("Invalid color: {}", color)
		}
	}
}

#[allow(dead_code)]
pub struct Gpu {
	//Screen is 160x144 pixels
	screen_data:	[[Color; 160]; 144],
	//scanline counter
	sl_count:		i16,
	//Background Palette Register
	bp:				u8,
}

impl Gpu {
	pub fn new() -> Gpu {
	    Gpu{screen_data: [[Color::BLACK; 160]; 144], sl_count: 0, bp: 0}
	}
	pub fn get_screen(&self) -> &[[Color; 160]; 144] {
		&self.screen_data
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
					self.draw_line(mem);
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
				status &= 0xFB;
			}
		}

		mem.wb(0xFF41, status);
	}
	fn is_lcd_enabled(&self, mem: &Memory) -> bool {
		(mem.rb(0xFF40) & (1 << 7)) > 0
	}

	fn draw_line(&mut self, mem: &mut Memory) {
		let control = mem.rb(0xFF40);
		if (control & 1) > 0 {
			self.draw_background(mem);
		}
		if (control & 2) > 0 {
			self.draw_sprites(mem);
		}
	}
	fn draw_background(&mut self, mem: &mut Memory) {
		let (scroll_y, scroll_x) = (mem.rb(0xFF42), mem.rb(0xFF43));
		let (window_y, window_x) = (mem.rb(0xFF4A), mem.rb(0xFF4B) - 7);

		let control = mem.rb(0xFF40);
		let tile_data_loc   = if (control & (1 << 4)) > 0 {0x8000} else {0x8800};
		let back_layout_loc = if (control & (1 << 3)) > 0 {0x9C00} else {0x9800};
		let wind_layout_loc = if (control & (1 << 6)) > 0 {0x9C00} else {0x9800};
		let using_window = (control & (1 << 5)) > 0 && window_y <= mem.rb(0xFF44);
		let background_loc: u16 = if using_window {wind_layout_loc} else {back_layout_loc};

		let line = mem.rb(0xFF44);
		let y_offset = if using_window {line - window_y} else {scroll_y + line};
		let tile_row = y_offset/8;

		for pixel in 0..160 {
			let mut x_offset = pixel + scroll_x;
			if using_window && pixel >= window_x {
				x_offset = pixel - window_x;
			}

			let tile_col = x_offset/8;
			let address = background_loc + tile_row as u16 *32 + tile_col as u16;

			let tile_loc = if tile_data_loc == 0x8000 {
				tile_data_loc + (mem.rb(address as u16) as u16)*16
			} else {
				tile_data_loc + ((mem.rb(address as u16) as i16 + 128) as u16 * 16)
			};
			let tile_line = (y_offset%8) as u16;

			let tile_data = [mem.rb(tile_loc + tile_line*2), mem.rb(tile_loc + tile_line*2 + 1)];
			let color_bit = 7 - x_offset%8;
			let color_id = ((tile_data[1] & (1 << color_bit)) >> (color_bit-1)) | 
						   ((tile_data[0] & (1 << color_bit)) >> color_bit);

			self.screen_data[line as usize][pixel as usize] = 
				Color::from_palette(color_id, mem.rb(0xFF47));
		}
	}
	#[allow(unused_variables)]
	fn draw_sprites(&mut self, mem: &mut Memory) {

	}
}