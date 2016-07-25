use std::fs::File;

use emulator::cartridge::Cartridge;
use emulator::mbc1::Mbc1;

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug)]
pub enum CartridgeType {
	ROM_ONLY, MBC1, MBC1_RAM, MBC1_RAM_BATTERY, MBC2, MBC2_BATTERY, ROM_RAM, ROM_RAM_BATTERY,
	MMM01, MMM01_RAM, MMM01_RAM_BATTERY, MBC3_TIMER_BATTERY, MBC3_TIMER_RAM_BATTERY, MBC3,
	MBC3_RAM, MBC3_RAM_BATTERY, MBC4, MBC4_RAM, MBC4_RAM_BATTERY, MBC5, MBC5_RAM,
	MBC5_RAM_BATTERY, MBC5_RUMBLE, MBC5_RUMBLE_RAM, MBC5_RUMBLE_RAM_BATTERY, MBC6,
	MBC7_SENSOR_RUMBLE_RAM_BATTERY, POCKET_CAMERA, BANDAI_TAMA5, HUC3, HUC1_RAM_BATTERY
}

impl CartridgeType {
	pub fn from_code(code: u8) -> Option<CartridgeType> {
		match code {
			0 => Some(CartridgeType::ROM_ONLY),
			1 => Some(CartridgeType::MBC1),
			_ => None
		}
	}
}

pub enum Mbc {
	EMPTY,
	NONE(Cartridge),
	MBC1(Mbc1)
}

impl Mbc {
	pub fn new(cartridge_type: CartridgeType) -> Mbc {
		match cartridge_type {
			CartridgeType::ROM_ONLY => Mbc::NONE(Cartridge::new()),
			CartridgeType::MBC1 	=> Mbc::MBC1(Mbc1::new()),
			_						=> panic!("Unimplemented cartridge type: {:?}", cartridge_type)
		}
	}
	pub fn rb(&self, address: usize) -> u8 {
		match *self {
			Mbc::EMPTY => panic!("Attempted to utilize empty cartridge"),
			Mbc::NONE(ref cart) => cart.rb(address),
			Mbc::MBC1(ref cart) => cart.rb(address)
		}
	}
	pub fn wb(&mut self, address: usize, val: u8) {
		match *self {
			Mbc::EMPTY => panic!("Attempted to utilize empty cartridge"),
			Mbc::NONE(ref mut cart) => cart.wb(address, val),
			Mbc::MBC1(ref mut cart) => cart.wb(address, val)
		}
	}
	pub fn load_game(&mut self, game_file: &mut File) -> usize {
		match *self {
			Mbc::EMPTY => panic!("Attempted to utilize empty cartridge"),
			Mbc::NONE(ref mut cart) => cart.load_game(game_file),
			Mbc::MBC1(ref mut cart) => cart.load_game(game_file)
		}
	}
}