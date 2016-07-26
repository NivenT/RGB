use std::fs::File;

use emulator::cartridge::Cartridge;
use emulator::mbc1::Mbc1;
use emulator::mbc2::Mbc2;
use emulator::mbc3::Mbc3;

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug)]
pub enum CartridgeType {
	ROM_ONLY, MBC1, MBC2, ROM_RAM, ROM_RAM_BATTERY, MMM01, MMM01_RAM, MMM01_RAM_BATTERY, MBC3,
	MBC4, MBC5, MBC6, MBC7_SENSOR_RUMBLE_RAM_BATTERY, POCKET_CAMERA, BANDAI_TAMA5, HUC3, 
	HUC1_RAM_BATTERY
}

impl CartridgeType {
	pub fn from_code(code: u8) -> Option<CartridgeType> {
		match code {
			0 		=> Some(CartridgeType::ROM_ONLY),
			1...3 	=> Some(CartridgeType::MBC1),
			5 | 6	=> Some(CartridgeType::MBC2),
			15...19	=> Some(CartridgeType::MBC3),
			_ 		=> None
		}
	}
}

pub enum Mbc {
	EMPTY,
	NONE(Cartridge),
	MBC1(Mbc1),
	MBC2(Mbc2),
	MBC3(Mbc3)
}

impl Mbc {
	pub fn new(cartridge_type: CartridgeType) -> Mbc {
		match cartridge_type {
			CartridgeType::ROM_ONLY => Mbc::NONE(Cartridge::new()),
			CartridgeType::MBC1 	=> Mbc::MBC1(Mbc1::new()),
			CartridgeType::MBC2 	=> Mbc::MBC2(Mbc2::new()),
			CartridgeType::MBC3 	=> Mbc::MBC3(Mbc3::new()),
			_						=> panic!("Unimplemented cartridge type: {:?}", cartridge_type)
		}
	}
	pub fn rb(&self, address: usize) -> u8 {
		match *self {
			Mbc::EMPTY => panic!("Attempted to utilize empty cartridge"),
			Mbc::NONE(ref cart) => cart.rb(address),
			Mbc::MBC1(ref cart) => cart.rb(address),
			Mbc::MBC2(ref cart) => cart.rb(address),
			Mbc::MBC3(ref cart) => cart.rb(address)
		}
	}
	pub fn wb(&mut self, address: usize, val: u8) {
		match *self {
			Mbc::EMPTY => panic!("Attempted to utilize empty cartridge"),
			Mbc::NONE(ref mut cart) => cart.wb(address, val),
			Mbc::MBC1(ref mut cart) => cart.wb(address, val),
			Mbc::MBC2(ref mut cart) => cart.wb(address, val),
			Mbc::MBC3(ref mut cart) => cart.wb(address, val)
		}
	}
	pub fn load_game(&mut self, game_file: &mut File) -> usize {
		match *self {
			Mbc::EMPTY => panic!("Attempted to utilize empty cartridge"),
			Mbc::NONE(ref mut cart) => cart.load_game(game_file),
			Mbc::MBC1(ref mut cart) => cart.load_game(game_file),
			Mbc::MBC2(ref mut cart) => cart.load_game(game_file),
			Mbc::MBC3(ref mut cart) => cart.load_game(game_file)
		}
	}
	pub fn step(&mut self, cycles: i16) {
		match *self {
			Mbc::MBC3(ref mut cart) => cart.step(cycles),
			_						=> {}
		}
	}
}