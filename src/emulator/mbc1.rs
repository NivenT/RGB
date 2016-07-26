use std::fs::File;
use std::io::Read;

pub struct Mbc1 {
	rom:		[u8; 0x200000], //2MB ROM
	ram:		[u8; 0x008000], //32KB RAM
	rom_bank:	u8,				
	ram_bank:	u8,
	using_ram:	bool,
	mode:		bool			//false for ROM and true for RAM		
}

impl Mbc1 {
	pub fn new() -> Mbc1 {
		Mbc1{rom: [0; 0x200000], ram: [0; 0x008000], rom_bank: 1, ram_bank: 0, 
				using_ram: false, mode: false}
	}
	pub fn rb(&self, address: usize) -> u8 {
		if address < 0x4000 {
			self.rom[address]
		} else if address < 0x8000 {
			self.rom[(self.rom_bank as usize-1)*0x4000 + address]
		} else if 0xA000 <= address && address < 0xC000 {
			if self.using_ram {
				self.ram[self.ram_bank as usize*0x2000 + address%0x2000] 
			} else {
				0
			}
		} else {
			panic!("Attempting to read from invalid MBC1 memory address: {:#X} ", address);
		}
	}
	pub fn wb(&mut self, address: usize, val: u8) {
		if address < 0x2000 {
			self.using_ram = (val & 0xF) == 0xA;
		} else if address < 0x4000 {
			self.rom_bank = (self.rom_bank & 0xE0) | (val & 0x1F);
			if self.rom_bank & 0x1F == 0 {
				self.rom_bank += 1;
			}
		} else if address < 0x6000 {
			let val = val & 0x3;
			if self.mode {
				self.ram_bank = val;
			} else {
				self.rom_bank |= val << 5;
			}
		} else if address < 0x8000 {
			self.mode = val%2 == 1;
			if self.mode {
				self.rom_bank &= !0x60;
			} else {
				self.ram_bank = 0;
			}
		} else if 0xA000 <= address && address < 0xC000 {
			if self.using_ram {
				self.ram[self.ram_bank as usize*0x2000 + address%0x2000] = val;
			}
		} else {
			panic!("Attempting to write to invalid MBC1 memory address: {:#X}", address);
		}
	}
	pub fn load_game(&mut self, game_file: &mut File) -> usize {
		game_file.read(&mut self.rom).unwrap()
	}
}