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

//returns size of ROM in KiB
pub fn get_rom_size(code: u8) -> Option<usize> {
	match code as u32 {
		n @ 0...7 => Some(2usize.pow(n+5)),
		_ => None
	}
}

//returns size of RAM in KiB
pub fn get_ram_size(code: u8) -> Option<usize> {
	match code {
		0 => Some(0),
		1 => Some(2),
		2 => Some(8),
		3 => Some(32),
		4 => Some(128),
		5 => Some(64),
		_ => None
	}
}