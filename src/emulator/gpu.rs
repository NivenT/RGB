use emulator::Memory;
use emulator::InterruptManager;

const SCANLINE_TOTAL_TIME: i16 = 456;
const SCANLINE_MODE2_OVER: i16 = 456-80;
const SCANLINE_MODE3_OVER: i16 = 456-80-172;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
	WHITE, LIGHT_GRAY, DARK_GRAY, BLACK,
	CGB(u8, u8, u8)
}

impl Color {
	pub fn to_f32(&self) -> Option<f32> {
		match *self {
			Color::WHITE => Some(255f32),
			Color::LIGHT_GRAY => Some(192f32),
			Color::DARK_GRAY => Some(96f32),
			Color::BLACK => Some(0f32),
			_ => None,
		}
	}

	fn from_gb_palette(id: u8, palette: u8) -> Color {
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
	fn from_cgb_palette_bgp(id: u8, number: u8, mem: &Memory) -> Color {
		let index = (8*number + 2*id) as usize;
		let data = mem.read_bgp(index) as u16 | (mem.read_bgp(index+1) as u16) << 8;
		let (red, green, blue) = (
			data & 0x1F,
			(data & 0x3E0) >> 5,
			(data & 0x7C00) >> 10
		);

		let scale = (0xFF as f32)/(0x1F as f32);
		Color::CGB((red as f32 * scale) as u8,
				   (green as f32 * scale) as u8,
				   (blue as f32 * scale) as u8)
	}
	fn from_cgb_palette_sp(id: u8, number: u8, mem: &Memory) -> Color {
		let index = (8*number + 2*id) as usize;
		let data = mem.read_sp(index) as u16 | (mem.read_sp(index+1) as u16) << 8;
		let (red, green, blue) = (
			data & 0x1F,
			(data & 0x3E0) >> 5,
			(data & 0x7C00) >> 10
		);

		let scale = (0xFF as f32)/(0x1F as f32);
		Color::CGB((red as f32 * scale) as u8,
				   (green as f32 * scale) as u8,
				   (blue as f32 * scale) as u8)
	}
}

pub struct Gpu {
	//Screen is 160x144 pixels
	screen_data:	[[Color; 160]; 144],
	//background priority for each pixel
	bg_priority:	[[u8; 160]; 144], //0b00000IIP where II is color id and P is priority (CGB only)
	//scanline counter
	sl_count:		i16
}

impl Gpu {
	pub fn new() -> Gpu {
	    Gpu {
	    	screen_data: [[Color::CGB(0,0,0); 160]; 144], 
	    	bg_priority: [[0; 160]; 144],
	    	sl_count: 0
	    }
	}
	pub fn get_screen(&self) -> &[[Color; 160]; 144] {
		&self.screen_data
	}
	pub fn get_scanline_count(&self) -> i16 {
		self.sl_count
	}
	pub fn step(&mut self, mem: &mut Memory, im: &InterruptManager, cycles: i16, cgb_mode: bool) {
		self.set_lcd_status(mem, im, cgb_mode);
		if self.is_lcd_enabled(mem) {
			self.sl_count -= cycles;
			if self.sl_count <= 0 {
				let line = (mem.rb(0xFF44) + 1)%154;
				mem.wl(line);

				self.sl_count = SCANLINE_TOTAL_TIME;
				if line == 144 {
					im.request_interrupt(mem, 0);
				} else if line < 144 {
					self.draw_line(mem, cgb_mode);
				}
			}
		}
	}
	fn set_lcd_status(&mut self, mem: &mut Memory, im: &InterruptManager, cgb_mode: bool) {
		let mut status = mem.rb(0xFF41);
		let line = mem.rb(0xFF44);
		let mode = status & 3;

		let mut request_interrupt = false;
		if !self.is_lcd_enabled(mem) {
			self.sl_count = SCANLINE_TOTAL_TIME;
			mem.wl(0);
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
			im.request_interrupt(mem, 1);
		}
		if line == mem.rb(0xFF45) && self.is_lcd_enabled(mem) {
			status = (status & 0xFB) | 4;
			if (status & (1 << 6)) > 0 {
				im.request_interrupt(mem, 1);
			} else {
				status &= 0xFB;
			}
		}

		let dma_info = mem.rb(0xFF55);
		if (status & 3) == 0 && dma_info & (1 << 7) > 0 && dma_info != 0xFF && cgb_mode {
			//H-Blank DMA
			let source =  (mem.rb(0xFF52) as u16 | ((mem.rb(0xFF51) as u16) << 8)) & 0xFFF0;
			let dest   = ((mem.rb(0xFF54) as u16 | ((mem.rb(0xFF53) as u16) << 8)) & 0x1FF0) | 0x8000;
			let length = 0x10*((dma_info & 0x7F) as u16+1);

			for i in 0..0x10 {
				let copy_val = mem.rb(source + length - i - 1);
				mem.wb(dest + length - i - 1, copy_val);
			}

			let length = (length/0x10 - 1) as u8;
			mem.wb(0xFF55, length.wrapping_sub(1) | (1 << 7));
		}

		mem.wb(0xFF41, status);
	}
	fn is_lcd_enabled(&self, mem: &Memory) -> bool {
		(mem.rb(0xFF40) & (1 << 7)) > 0
	}
	fn draw_line(&mut self, mem: &Memory, cgb_mode: bool) {
		self.draw_tiles(mem, cgb_mode);
		if (mem.rb(0xFF40) & 2) > 0 {
			self.draw_sprites(mem, cgb_mode);
		}
	}
	fn draw_tiles(&mut self, mem: &Memory, cgb_mode: bool) {
		let (scroll_y, scroll_x) = (mem.rb(0xFF42), mem.rb(0xFF43));
		let (window_y, window_x) = (mem.rb(0xFF4A), mem.rb(0xFF4B).wrapping_sub(7));

		let (control, line) = (mem.rb(0xFF40), mem.rb(0xFF44));
		let tile_data_loc   = if (control & (1 << 4)) > 0 {0x8000} else {0x8800};
		let back_layout_loc = if (control & (1 << 3)) > 0 {0x9C00} else {0x9800};
		let wind_layout_loc = if (control & (1 << 6)) > 0 {0x9C00} else {0x9800};
		let using_window = (control & (1 << 5)) > 0 && window_y <= line;
		let background_loc: u16 = if using_window {wind_layout_loc} else {back_layout_loc}; 

		if !using_window && control & 1 == 0 {
			for pixel in 0..160u8 {
				self.bg_priority[line as usize][pixel as usize] = 0;
			}
			return;
		}

		let y_offset = if using_window {line - window_y} else {scroll_y.wrapping_add(line)};
		let tile_row = y_offset/8;

		for pixel in 0..160u8 {
			let mut x_offset = pixel.wrapping_add(scroll_x);
			if using_window && pixel >= window_x {
				x_offset = pixel - window_x;
			}

			let tile_col = x_offset/8;
			let address = background_loc + tile_row as u16*32 + tile_col as u16;

			let tile_loc = if tile_data_loc == 0x8000 {
				tile_data_loc + (mem.read_vram(address, false) as u16)*16
			} else {
				tile_data_loc + ((mem.read_vram(address, false) as i8 as i16 + 128) as u16 * 16)
			};
			let tile_attributes = if cgb_mode {mem.read_vram(address, true)} else {0};

			let (x_flip, y_flip) = (tile_attributes & (1 << 5) > 0, tile_attributes & (1 << 6) > 0);
			self.bg_priority[line as usize][pixel as usize] = (tile_attributes & (1 << 7)) >> 7;

			let tile_line = if y_flip {
				7 - y_offset%8
			} else {
				y_offset%8 
			} as u16;

			let tile_data = if cgb_mode {
				let bank = tile_attributes & (1 << 3) > 0;
				[mem.read_vram(tile_loc+tile_line*2, bank),
				 mem.read_vram(tile_loc+tile_line*2+1, bank)]
			} else {
				[mem.rb(tile_loc+tile_line*2), mem.rb(tile_loc+tile_line*2+1)]
			};

			let color_bit = if x_flip {
				x_offset%8
			} else {
				7 - x_offset%8
			};

			let color_id = if color_bit == 0 {
				((tile_data[1] & 1) << 1) | (tile_data[0] & 1)
			} else {
				((tile_data[1] & (1 << color_bit)) >> (color_bit-1)) | 
				((tile_data[0] & (1 << color_bit)) >> color_bit)
			};
			self.bg_priority[line as usize][pixel as usize] |= color_id << 1;

			self.screen_data[line as usize][pixel as usize] = if cgb_mode {
				let palette_number = tile_attributes & 7;
				Color::from_cgb_palette_bgp(color_id, palette_number, mem)
			} else {
				Color::from_gb_palette(color_id, mem.rb(0xFF47))
			}				
		}
	}
	fn bg_has_priority(&self, mem: &Memory, line: usize, pixel: usize, behind_bg: bool) -> bool {
		mem.rb(0xFF40) & 1 > 0 &&
		(self.bg_priority[line][pixel] & 1 > 0 || behind_bg) &&
		self.bg_priority[line][pixel] > 1
	}
	fn draw_sprites(&mut self, mem: &Memory, cgb_mode: bool) {
		let control = mem.rb(0xFF40);
		let large_sprites = (control & (1 << 2)) > 0;

		for sprite in (0..40).rev() {
			let offset = sprite*4;
			let (x_pos, y_pos) = (mem.rb(0xFE00+offset+1).wrapping_sub(8),
								  mem.rb(0xFE00+offset).wrapping_sub(16));
			let sprite_loc = mem.rb(0xFE00+offset+2);
			let attributes = mem.rb(0xFE00+offset+3);

			let (x_flip, y_flip) = ((attributes & (1 << 5)) > 0, (attributes & (1 << 6)) > 0);
			let line = mem.rb(0xFF44);

			let y_size = if large_sprites {16} else {8};
			if y_pos <= line && line < y_pos + y_size {
				let sprite_line = if y_flip {y_size+y_pos-line-1} else {line - y_pos};
				let address = 0x8000 + sprite_loc as u16*16 + sprite_line as u16*2;
				let data = if cgb_mode {
					let bank = attributes & (1 << 3) > 0;
					[mem.read_vram(address, bank), mem.read_vram(address+1, bank)]
				} else {
					[mem.rb(address), mem.rb(address+1)]
				};
				for color_bit in 0..8 {
					let color_id = if color_bit == 0 {
						((data[1] & 1) << 1) | (data[0] & 1)
					} else {
						((data[1] & (1 << color_bit)) >> (color_bit-1)) | 
						((data[0] & (1 << color_bit)) >> color_bit)
					};

					let color = if cgb_mode {
						let palette_number = attributes & 7;
						Color::from_cgb_palette_sp(color_id, palette_number, mem)
					} else {
						let palette_address = if (attributes & (1 << 4)) > 0 {0xFF49} else {0xFF48};
						Color::from_gb_palette(color_id, mem.rb(palette_address))
					};

					let behind_bg = attributes & (1 << 7) > 0;
					if color_id != 0 {
						let pixel = if x_flip {
							x_pos.wrapping_add(color_bit)
						} else {
							x_pos.wrapping_add(7-color_bit)
						};

						if pixel < 160 && !self.bg_has_priority(mem, line as usize, pixel as usize, behind_bg) {
							self.screen_data[line as usize][pixel as usize] = color;
						}
					}
				}
			}
		}
	}
}