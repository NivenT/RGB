use std::fs::File;
use std::io::Read;

const RTC_RESETS: [u8; 3] = [60, 60, 24];

pub struct Mbc3 {
	rom:		Vec<u8>, //2MB ROM
	ram:		Vec<u8>, //32KB RAM
	rtc:		[u8; 5], //5 clock registers
	rom_bank:	u8,				
	ram_bank:	u8,
	rtc_reg:	u8,
	using_ram:	bool,
	mode:		bool,	//false for ROM and true for RTC
	clock:		u16,	//32,768 Hz (update every 4194304/32768=128 cycles)
	counter:	i16,	//increment clock when counter hits 0
	prev_val:	u8,		//Last value written to 0x6000-0x7FFF
	using_clk:	bool
}

impl Mbc3 {
	pub fn new() -> Mbc3 {
		Mbc3{rom: vec![0; 0x200000], ram: vec![0; 0x008000], rtc: [0; 5],
				rom_bank: 1, ram_bank: 0, rtc_reg: 0,
				using_ram: false, mode: false,
				clock: 0, counter: 0, prev_val: 1, using_clk: true}
	}
	pub fn rb(&self, address: usize) -> u8 {
		if address < 0x4000 {
			self.rom[address]
		} else if address < 0x8000 {
			self.rom[(self.rom_bank as usize-1)*0x4000 + address]
		} else if 0xA000 <= address && address < 0xC000 {
			if self.using_ram {
				if self.mode {
					self.rtc[self.rtc_reg as usize]
				} else {
					self.ram[self.ram_bank as usize*0x2000 + address%0x2000]
				} 
			} else {
				0
			}
		} else {
			panic!("Attempting to read from invalid MBC3 memory address: {:#X} ", address);
		}
	}
	pub fn wb(&mut self, address: usize, val: u8) {
		if address < 0x2000 {
			self.using_ram = (val & 0xF) == 0xA;
		} else if address < 0x4000 {
			self.rom_bank = if (val & 0x7F) > 0 {val & 0x7F} else {1};
		} else if address < 0x6000 {
			if val < 4 {
				self.mode = false;
				self.ram_bank = val;
			} else if 0x8 <= val && val < 0xD {
				self.mode = true;
				self.rtc_reg = val-0x8;
			}
		} else if address < 0x8000 {
			if self.prev_val == 0 && val == 1 {
				self.using_clk = !self.using_clk;
			}
			self.prev_val = val;
		} else if 0xA000 <= address && address < 0xC000 {
			if self.using_ram {
				if self.mode {
					self.rtc[self.rtc_reg as usize] = val;
				} else {
					self.ram[self.ram_bank as usize*0x2000 + address%0x2000] = val;
				}
			}
		} else {
			panic!("Attempting to write to invalid MBC3 memory address: {:#X}", address);
		}
	}
	fn increment_rtc(&mut self, index: usize) {
		if index == 3 {
			let mut day_counter = self.rtc[3] as u16 | 0x100*(self.rtc[4] & 1) as u16;
			if day_counter == 0x1FF {
				day_counter = 0;
				self.rtc[4] |= 1 << 7;
			} else {
				day_counter += 1;
			}
			self.rtc[3] = (day_counter & 0xFF) as u8;
			self.rtc[4] = (self.rtc[4] & 0xFE) | ((day_counter >> 8) as u8);
		} else {
			self.rtc[index] += 1;
			if self.rtc[index] == RTC_RESETS[index] {
				self.rtc[index] = 0;
				self.increment_rtc(index+1);
			}
		}
	}
	pub fn step(&mut self, cycles: i16) {
		if self.rtc[4] & (1 << 6) == 0 {
			self.counter -= cycles;
		}
		if self.counter <= 0 {
			self.counter = 128;
			self.clock += 1;

			if self.clock == 32768 {
				self.clock = 0;
				if self.using_clk {
					self.increment_rtc(0);
				}
			}
		}
	}
	pub fn load_game(&mut self, game_file: &mut File) -> usize {
		game_file.read(&mut self.rom).unwrap()
	}
}