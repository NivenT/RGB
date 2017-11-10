use std::fs::File;
use std::io::{Write, Read};

pub struct Mbc5 {
	//Unknown error occurs when using an array instead of a Vec
	rom:		Vec<u8>, //8MB ROM
	ram:		Vec<u8>, //128KB RAM
	rom_bank:	u16,				
	ram_bank:	u8,
	using_ram:	bool	
}

impl Mbc5 {
	pub fn new() -> Mbc5 {
		Mbc5{rom: vec![0; 0x800000], ram: vec![0; 0x020000], rom_bank: 0, ram_bank: 0, 
			using_ram: false}
	}
	pub fn rb(&self, address: usize) -> u8 {
		if address < 0x4000 {
			self.rom[address]
		} else if address < 0x8000 {
			self.rom[self.rom_bank as usize * 0x4000 | (address & 0x3FFF)]
		} else if 0xA000 <= address && address < 0xC000 {
			if self.using_ram {
				self.ram[self.ram_bank as usize*0x2000 + address%0x2000] 
			} else {
				0
			}
		} else {
			panic!("Attempting to read from invalid MBC5 memory address: {:#X} ", address);
		}
	}
	pub fn wb(&mut self, address: usize, val: u8) {
		if address < 0x2000 {
			self.using_ram = (val & 0xF) == 0xA;
		} else if address < 0x3000 {
			self.rom_bank = (self.rom_bank & 0x100) | val as u16;
		} else if address < 0x4000 {
			self.rom_bank = (((val & 1) as u16) << 8) | (self.rom_bank & 0xFF);
		} else if address < 0x6000 {
			self.ram_bank = val & 0xF;
		} else if 0xA000 <= address && address < 0xC000 {
			if self.using_ram {
				self.ram[self.ram_bank as usize*0x2000 + address%0x2000] = val;
			}
		} else {
			//panic!("Attempting to write to invalid MBC5 memory address: {:#X}", address);
		}
	}
	pub fn load_game(&mut self, game_file: &mut File) -> usize {
		game_file.read(&mut self.rom).unwrap()
	}
	pub fn load_sav(&mut self, save_file: &mut File) -> usize {
		save_file.read(&mut self.ram).unwrap()
	}
	pub fn save_game(&mut self, save_file: &mut File) -> usize {
		save_file.write(&self.ram).unwrap()
	}
}