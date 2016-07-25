use std::fs::File;

use emulator::cartridge::Cartridge;

pub enum Mbc {
	NONE(Cartridge)
}

impl Mbc {
	pub fn new() -> Mbc {
		Mbc::NONE(Cartridge::new())
	}
	pub fn rb(&self, address: usize) -> u8 {
		match *self {
			Mbc::NONE(ref cart) => cart.rb(address)
		}
	}
	pub fn wb(&mut self, address: usize, val: u8) {
		match *self {
			Mbc::NONE(ref mut cart) => cart.wb(address, val)
		}
	}
	pub fn load_game(&mut self, game_file: &mut File) -> usize {
		match *self {
			Mbc::NONE(ref mut cart) => cart.load_game(game_file)
		}
	}
}