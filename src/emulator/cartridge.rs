use std::fs::File;
use std::io::Read;

//Simply cartridge with no memory banking
pub struct Cartridge {
	rom:	[u8; 0x8000] //32KB ROM
}

impl Cartridge {
	pub fn new() -> Cartridge {
		Cartridge{rom: [0; 0x8000]}
	}
	pub fn rb(&self, address: usize) -> u8 {
	    self.rom[address as usize]
	}
	pub fn wb(&mut self, address: usize, val: u8) {
		self.rom[address as usize] = val;
	}
	pub fn load_game(&mut self, game_file: &mut File) -> usize {
		game_file.read(&mut self.rom).unwrap()
	}
}