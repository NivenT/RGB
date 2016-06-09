use std::fs::File;
use std::io::prelude::*;

use emulator::registers::Registers;
use emulator::memory::Memory;
use emulator::gpu::Gpu;
use emulator::instructions::*;
use emulator::rom_info::*;

use super::super::debug_output;

pub struct Emulator {
	debug_file:		File,
	clock:			u64,

	pub mem:		Memory,
	pub gpu:		Gpu,
	pub controls: 	[u8; 8],
	pub regs:		Registers
}

impl Emulator {
	pub fn new() -> Emulator {
		let mut memory = Memory::new();
		for i in 0..256 {
			memory.wb(i, BIOS[i as usize]);
		}
		Emulator{debug_file: File::create("debug.txt").unwrap(), clock: 0, mem: memory,
					gpu: Gpu::new(), controls: [0; 8], regs: Registers::new()}
	}
	pub fn set_controls(&mut self, controls: Vec<u8>) {
		for i in 0..8 {
			self.controls[i] = controls[i];
		}
	}
	pub fn load_game(&mut self, path: String) {
		println!("Loading game from \"{}\"...", path);
		let mut game_file = File::open(path).unwrap();

		let size = game_file.read(&mut self.mem.cart).unwrap();
		println!("Game has a size of {} bytes ({} KiB)", size, size/1024);

		let header = &self.mem.cart[..0x150];
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

		println!("Successfully loaded {}", title);
	}
	pub fn emulate_cycle(&mut self) {
		let address = self.regs.pc;
		let opcode = self.mem.rb(self.regs.pc); self.regs.pc += 1;
		let instruction = INSTRUCTIONS[opcode as usize];

		let operand = if instruction.operand_length == 1 {
			self.mem.rb(self.regs.pc) as u16
		} else {
			self.mem.rw(self.regs.pc)
		};
		self.regs.pc += instruction.operand_length as u16;

		let cycles: u64;
		if let Some(func) = instruction.func {
			let debug_info = format!("Running instruction {:#X} ({} | {}) with operand {:#X} at address ({:#X})\n\t{:?}\n",
								opcode, instruction.name, instruction.operand_length, operand, address, self.regs);
			unsafe {if debug_output {println!("{}", debug_info);}}
			let _ = write!(self.debug_file, "{}\n", debug_info);

			cycles = func(self, operand);
		} else {
			let debug_info = format!("\nUnimplemented function at memory address ({:#X}) [{:#X} ({} | {})] called with operand {:#X}\n", 
				address, opcode, instruction.name, instruction.operand_length, operand);
			let _ = write!(self.debug_file, "{}", debug_info);
			println!("{}", debug_info);
			panic!("");
		}
		
		self.clock += cycles;
		self.gpu.step(&mut self.mem, cycles as i16);
	}
}