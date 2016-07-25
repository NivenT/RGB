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