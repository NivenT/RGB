use std::fs::File;
use std::io::prelude::*;

use emulator::registers::Registers;
use emulator::instructions::*;
use emulator::rom_info::*;

#[allow(dead_code)]
pub struct Emulator {
	pub memory:		[u8; 65536],
	pub controls: 	[u8; 8],
	pub regs:		Registers
}

#[allow(dead_code)]
impl Emulator {
	pub fn new() -> Emulator {
		let mut memory = [0; 65536];
		for i in 0..256 {
			memory[i] = BIOS[i];
		}
		Emulator{memory: memory, controls: [0; 8], regs: Registers::new()}
	}
	pub fn set_controls(&mut self, controls: Vec<u8>) {
		for i in 0..8 {
			self.controls[i] = controls[i];
		}
	}
	pub fn load_game(&mut self, path: String) {
		println!("Loading game from \"{}\"...", path);
		let mut game_file = File::open(path).unwrap();
		let mut contents: Vec<u8> = Default::default();

		let size = game_file.read_to_end(&mut contents).unwrap();
		println!("Game has a size of {} bytes ({} KiB)", size, size/1024);

		let header = &contents[..0x150];
		let title = String::from_utf8_lossy(&header[0x134..0x144]);
		println!("The title of the game is {}", title);

		let sgb_flag = header[0x146];
		if sgb_flag > 0 {
			println!("{} supports Super GameBoy functions", title);
		} else {
			println!("{} does not support Super GameBoy functions", title);
		}

		let cartridge_type = header[0x147];
		let cartridge_type = match CartridgeType::from_code(cartridge_type) {
			Some(t) => t,
			None  	=> panic!("Unknown cartridge type: {}", cartridge_type)
		};
		println!("The cartridge type is {:?}", cartridge_type);

		let rom_size = header[0x148];
		let rom_size = match get_rom_size(rom_size) {
			Some(size) 	=> size * 1024,
			None 		=> panic!("Unkown ROM size type: {}", rom_size)
		};
		println!("{} has {} bytes ({} KiB) used for ROM", title, rom_size, rom_size/1024);

		let ram_size = header[0x149];
		let ram_size = match get_ram_size(ram_size) {
			Some(size)	=> size * 1024,
			None		=> panic!("Unknown RAM size type: {}", ram_size)
		};
		println!("{} has {} bytes ({} KiB) of external RAM", title, ram_size, ram_size/1024);

		let destination_code = header[0x14A];
		if destination_code > 0 {
			println!("This is the non-Japanese version of {}", title);
		} else {
			println!("This is the Japanese version of {}", title);
		}

		for i in 0..contents.len() {
			self.memory[i] = contents[i];
		}
		println!("Successfully loaded {}", title);
	}
	pub fn emulate_cycle(&mut self) {
		let opcode = self.memory[self.regs.pc as usize]; self.regs.pc += 1;
		let instruction = INSTRUCTIONS[opcode as usize];

		let mut operand = self.memory[self.regs.pc as usize] as u16;
		if instruction.operand_length == 2 {
			operand = (operand << 8) | self.memory[self.regs.pc as usize+1] as u16;
		}

		if let Some(func) = instruction.func {
			func(self, operand);
		} else {
			println!("Unimplemented function at memory address {:#X} [{:#X} ({} | {})] called with operand {:#X}", 
				self.regs.pc-1, opcode, instruction.name, instruction.operand_length, operand);
			panic!("");
		}
		self.regs.pc += instruction.operand_length as u16;
	}
}