use std::fs::File;
use std::io::Read;

pub struct Mbc2 {
	rom:		[u8; 0x40000], //256KB ROM
	ram:		[u8; 0x00100], //512x4bits (256 bytes) RAM
	rom_bank:	u8,	
	using_ram:	bool	
}

impl Mbc2 {
	pub fn new() -> Mbc2 {
		Mbc2{rom: [0; 0x40000], ram: [0; 0x00100], rom_bank: 1, using_ram: false}
	}
	pub fn rb(&self, address: usize) -> u8 {
		if address < 0x4000 {
			self.rom[address]
		} else if address < 0x8000 {
			self.rom[(self.rom_bank as usize-1)*0x4000 + address]
		} else if 0xA000 <= address && address < 0xA200 {
			if self.using_ram {
				if address >= 0xA100 {
					self.ram[address%0x0100] >> 4
				} else {
					self.ram[address%0x0100] & 0x0F
				}
			} else {
				0
			}
		} else {
			panic!("Attempting to read from invalid MBC2 memory address: {:#X} ", address);
		}
	}
	pub fn wb(&mut self, address: usize, val: u8) {
		if address < 0x2000 {
			if address & 0x100 == 0 {
				self.using_ram = (val & 0xF) == 0xA;
			}
		} else if address < 0x4000 {
			if address & 0x100 > 0 {
				self.rom_bank = val & 0xF;
			}
		} else if 0xA000 <= address && address < 0xA200 {
			let ram_address = address%0x0100;
			if self.using_ram {
				if address >= 0xA100 {
					self.ram[ram_address] = (val << 4) | (self.ram[ram_address] & 0x0F);
				} else {
					self.ram[ram_address] = (self.ram[ram_address] & 0xF0) | (val & 0x0F);
				}
			}
		} else {
			panic!("Attempting to write to invalid MBC2 memory address: {:#X}", address);
		}
	}
	pub fn load_game(&mut self, game_file: &mut File) -> usize {
		game_file.read(&mut self.rom).unwrap()
	}
}