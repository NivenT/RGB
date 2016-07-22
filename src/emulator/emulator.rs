use std::fmt;
use std::fs::{OpenOptions, File};
use std::io::SeekFrom;
use std::io::prelude::*;

use emulator::gpu::Gpu;
use emulator::interrupts::InterruptManager;
use emulator::instructions::*;
use emulator::registers::*;
use emulator::rom_info::*;
use emulator::memory::*;

use super::super::programstate::*;

#[allow(dead_code)]
pub struct Emulator {
	debug_file:		File,
	clock:			u64,
	interrupts:		InterruptManager,
	controls: 		[u8; 8],

	pub mem:		Memory,
	pub gpu:		Gpu,
	pub regs:		Registers,
	pub halted:		bool
}

impl fmt::Debug for Emulator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let _ = write!(f, "*****EMULATOR DEBUG INFO*****\n");
		unsafe {
			let _ = write!(f, "AF:           {:#X}\n", *self.regs.af_immut());
			let _ = write!(f, "BC:           {:#X}\n", *self.regs.bc_immut());
			let _ = write!(f, "DE:           {:#X}\n", *self.regs.de_immut());
			let _ = write!(f, "HL:           {:#X}\n", *self.regs.hl_immut());
			let _ = write!(f, "SP:           {:#X}\n",  self.regs.sp);
			let _ = write!(f, "PC:           {:#X}\n",  self.regs.pc);
			let _ = write!(f, "\n");
			let _ = write!(f, "ZERO:         {}\n", self.regs.get_flag(ZERO_FLAG));
			let _ = write!(f, "NEGATIVE:     {}\n", self.regs.get_flag(NEGATIVE_FLAG));
			let _ = write!(f, "HALFCARRY:    {}\n", self.regs.get_flag(HALFCARRY_FLAG));
			let _ = write!(f, "CARRY:        {}\n", self.regs.get_flag(CARRY_FLAG));
			let _ = write!(f, "\n");
			let _ = write!(f, "IF:           {:#X}\n", self.mem.rb(0xFF0F));
			let _ = write!(f, "IE:           {:#X}\n", self.mem.rb(0xFFFF));
			let _ = write!(f, "IME:          {}\n", self.interrupts.ime);
			let _ = write!(f, "\n");
			let _ = write!(f, "SL_COUNT:     {}\n", self.gpu.get_scanline_count());
			let _ = write!(f, "LCD STATUS:   {:#b}\n", self.mem.rb(0xFF41));
			let _ = write!(f, "LCD CONTROL:  {:#b}\n", self.mem.rb(0xFF40));
			let _ = write!(f, "\n");
			let _ = write!(f, "JOYPAD STATE: {:#b}\n", self.mem.rb(0xFF00));
		}
		write!(f, "*****************************")
	}
}

#[allow(dead_code)]
impl Emulator {
	pub fn new() -> Emulator {
		let mut memory = Memory::new();
		for i in 0..256 {
			memory.wb(i, BIOS[i as usize]);
		}
		memory.wk(0xF);

		let debug_file = OpenOptions::new().read(true)
									  	   .write(true)
									  	   .truncate(true)
										   .create(true)
							   			   .open("debug.txt")
									       .unwrap();
		Emulator{debug_file: debug_file, clock: 0, mem: memory, gpu: Gpu::new(), 
					controls: [0; 8], regs: Registers::new(), halted: false,
					interrupts: InterruptManager::new()}
	}
	pub fn set_controls(&mut self, controls: Vec<u8>) {
		for i in 0..8 {
			self.controls[i] = controls[i];
		}
	}
	#[allow(unused_variables)]
	pub fn load_game(&mut self, path: String) {
		println!("Loading game from \"{}\"...", path);
		let mut game_file = File::open(path).unwrap();
		
		let size = game_file.read(&mut self.mem.cart).unwrap();
		//println!("Game has a size of {} bytes ({} KiB)", size, size/1024);
		
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
			None  	=> panic!("Unknown cartridge type: {:?}", cartridge_type)
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

		/*
		let destination_code = header[0x14A];
		if destination_code > 0 {
			println!("This is the non-Japanese version of {}", title);
		} else {
			println!("This is the Japanese version of {}", title);
		}
		*/
		println!("Successfully loaded {}\n", title);
	}
	pub fn enable_interrupts(&mut self) {
		self.interrupts.ime = true;
	}
	pub fn disable_interrupts(&mut self) {
		self.interrupts.ime = false;
	}
	pub fn update_keys(&mut self, key: u8, pressed: bool) {
		let mut key_state = self.mem.rb(0xFF00);
		for i in 0..8 {
			if self.controls[i] == key {
				let col = if i < 4 {1 << 4} else {1 << 5};
				let row = 1 << i%4;
				if key_state & col > 0 {
					if pressed {
						if key_state & row > 0 {
							self.interrupts.request_interrupt(&mut self.mem, 4);
						}
						key_state &= !row;
					} else {
						key_state |= row;
					}
				}
			}
		}
		self.mem.wk(key_state);
	}
	pub fn step(&mut self, state: &mut ProgramState) -> u64 {
		let cycles = if !self.halted {self.emulate_cycle(state)} else {4};
		self.gpu.step(&mut self.mem, &self.interrupts, cycles as i16);
		if self.interrupts.step(&mut self.mem, &mut self.regs) {
			self.halted = false;
		}

		if self.regs.pc == 0x100 {
			self.mem.finished_with_bios();
			/*
			unsafe {
				println!("AF = {:#X}", *self.regs.af());
				println!("BC = {:#X}", *self.regs.bc());
				println!("DE = {:#X}", *self.regs.de());
				println!("HL = {:#X}", *self.regs.hl());
				println!("SP = {:#X}",  self.regs.sp);
				for i in vec![0xFF05, 0xFF06, 0xFF07, 0xFF10, 0xFF11, 0xFF12, 0xFF14, 0xFF16, 0xFF17, 0xFF19, 0xFF1A, 0xFF1B, 0xFF1C, 0xFF1E, 0xFF20, 0xFF21, 0xFF22, 0xFF23, 0xFF24, 0xFF25, 0xFF26, 0xFF40, 0xFF42, 0xFF43, 0xFF45, 0xFF47, 0xFF48, 0xFF49, 0xFF4A, 0xFF4B, 0xFFFF] {
					println!("[{:#X}] = {:#X}", i, self.mem.rb(i));
				}
			}
			panic!("check for correctness");
			*/
		}
		cycles
	}

	fn emulate_cycle(&mut self, state: &mut ProgramState) -> u64 {
		let address = self.regs.pc;
		let opcode = self.mem.rb(self.regs.pc); self.regs.pc += 1;
		let instruction = INSTRUCTIONS[opcode as usize];

		let operand = if instruction.operand_length == 1 {
			self.mem.rb(self.regs.pc) as u16
		} else {
			self.mem.rw(self.regs.pc)
		};
		self.regs.pc += instruction.operand_length;

		let cycles: u64;
		if let Some(func) = instruction.func {
			let debug_info = format!("Running instruction {:#X} ({} | {}) with operand {:#X} at address ({:#X})\n{:?}\n",
								opcode, instruction.name, instruction.operand_length, operand, address, self);
			if state.debug {println!("{}", debug_info);}
			//self.update_debug_file(debug_info); //store debug info in a file

			cycles = func(self, operand);
		} else {
			let debug_info = format!("\nUnimplemented instruction at memory address ({:#X}) [{:#X} ({} | {})] called with operand {:#X}\n", 
				address, opcode, instruction.name, instruction.operand_length, operand);
			println!("{}", debug_info);
			self.update_debug_file(debug_info);
			panic!("");
		}
		
		self.clock += cycles;
		cycles
	}
	fn update_debug_file(&mut self, msg: String) {
		const MAX_SIZE: usize = 1024 * 1024; //File at most 1MiB
		let mut buf = vec![];

		let _ = write!(self.debug_file, "{}\n", msg);

		let _ = self.debug_file.seek(SeekFrom::Start(0));
		let size = match self.debug_file.read_to_end(&mut buf) {
			Ok(len)  => len,
			Err(msg) => panic!("{}", msg)
		};
		if size > MAX_SIZE {
			let _ = self.debug_file.set_len(0);
			/*
			let _ = self.debug_file.write(&buf[buf.len()-MAX_SIZE..]);
			let _ = self.debug_file.set_len(MAX_SIZE as u64);
			*/
		}
		let _ = self.debug_file.seek(SeekFrom::End(0));
	}
}