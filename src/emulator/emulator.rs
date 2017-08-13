use std::fmt;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::collections::HashSet;

use emulator::Memory;
use emulator::Gpu;
use emulator::InterruptManager;
use emulator::Timers;
use emulator::mbc::*;
use emulator::instructions::*;
use emulator::registers::*;
use emulator::rom_info::*;

use super::super::programstate::*;

fn to_save(game: String) -> String {
	let dot_pos = game.rfind('.').unwrap();
	// Not sure how regular .sav files are saved so these are .rsav
	game[..dot_pos].to_string() + ".rsav"
}

fn to_null_terminated(bytes: &[u8]) -> String {
	String::from_utf8_lossy(&bytes.iter()
								  .map(|b| *b)
	 							  .take_while(|&b| b > 0)
	 							  .collect::<Vec<_>>())
							.to_string()
}

pub struct Emulator {
	clock:			u64,
	interrupts:		InterruptManager,
	controls: 		[u8; 8],
	timers:			Timers,
	cgb_mode:		bool,

	pub(in emulator) mem: Memory,
	pub(in emulator) gpu: Gpu,
	pub(in emulator) regs: Registers,
	pub(in emulator) halted: bool,
	pub(in emulator) stopped: bool
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
			let _ = write!(f, "SCANLINE:     {}\n", self.mem.rb(0xFF44));
			let _ = write!(f, "LCD STATUS:   {:#b}\n", self.mem.rb(0xFF41));
			let _ = write!(f, "LCD CONTROL:  {:#b}\n", self.mem.rb(0xFF40));
			let _ = write!(f, "\n");
			let _ = write!(f, "DIV:          {:#X}\n", self.mem.rb(0xFF04));
			let _ = write!(f, "TIMA:         {:#X}\n", self.mem.rb(0xFF05));
			let _ = write!(f, "TMA:          {:#X}\n", self.mem.rb(0xFF06));
			let _ = write!(f, "TAC:          {:#X}\n", self.mem.rb(0xFF07));
		}
		write!(f, "*****************************")
	}
}

impl Emulator {
	pub fn new() -> Emulator {
		Emulator {
			clock: 0, 
			mem: Memory::new(), 
			gpu: Gpu::new(), 
			controls: [0; 8], 
			regs: Registers::new(), 
			halted: false, 
			timers: Timers::new(),
			interrupts: InterruptManager::new(), 
			stopped: false, 
			cgb_mode: false
		}
	}
	pub fn set_controls(&mut self, controls: Vec<u8>) {
		for i in 0..8 {
			self.controls[i] = controls[i];
		}
	}
	pub fn load_bios(&mut self, path: String) {
		println!("Loading BIOS from \"{}\"...", path);
		match File::open(path) {
			Ok(mut bios_file) => {
				println!("Found BIOS");
				let _ = bios_file.read_to_end(&mut self.mem.bios);
				println!("Successfully loaded bios\n");
			},
			Err(_) => {
				println!("Could not find BIOS");
				println!("Manually initializing emulator...");

				unsafe {
					*self.regs.af() = 0x11B0;
					*self.regs.bc() = 0x0013;
					*self.regs.de() = 0x00D8;
					*self.regs.hl() = 0x014D;
				}
				self.regs.sp = 0xFFFE;
				self.regs.pc = 0x0100;

 				self.mem.wb(0xFF05, 0x00);
				self.mem.wb(0xFF06, 0x00);
				self.mem.wb(0xFF07, 0x00);
				self.mem.wb(0xFF10, 0x80);
				self.mem.wb(0xFF11, 0xBF);
				self.mem.wb(0xFF12, 0xF3);
				self.mem.wb(0xFF14, 0xBF);
				self.mem.wb(0xFF16, 0x3F);
				self.mem.wb(0xFF17, 0x00);
				self.mem.wb(0xFF19, 0xBF);
				self.mem.wb(0xFF1A, 0x7F);
				self.mem.wb(0xFF1B, 0xFF);
				self.mem.wb(0xFF1C, 0x9F);
				self.mem.wb(0xFF1E, 0xBF);
				self.mem.wb(0xFF20, 0xFF);
				self.mem.wb(0xFF21, 0x00);
				self.mem.wb(0xFF22, 0x00);
				self.mem.wb(0xFF23, 0xBF);
				self.mem.wb(0xFF24, 0x77);
				self.mem.wb(0xFF25, 0xF3);
				self.mem.wb(0xFF26, 0xF1);
				self.mem.wb(0xFF40, 0x91);
				self.mem.wb(0xFF42, 0x00);
				self.mem.wb(0xFF43, 0x00);
				self.mem.wb(0xFF45, 0x00);
				self.mem.wb(0xFF47, 0xFC);
				self.mem.wb(0xFF48, 0xFF);
				self.mem.wb(0xFF49, 0xFF);
				self.mem.wb(0xFF4A, 0x00);
				self.mem.wb(0xFF4B, 0x00);
				self.mem.wb(0xFFFF, 0x00);

				self.mem.finished_with_bios();
				println!("Emulator initialized\n");
			}
		}
	}
	pub fn load_game(&mut self, path: String) {
		println!("Loading game from \"{}\"...", path);
		let mut game_file = File::open(path.clone()).unwrap();
		
		let mut header = [0; 0x150];
		let _ = game_file.read(&mut header).unwrap();
		let _ = game_file.seek(SeekFrom::Start(0));

		let title = to_null_terminated(&header[0x134..0x144]);

		println!("The title of the game is {}", title);
		
		let cartridge_type = header[0x147];
		let cartridge_type = match CartridgeType::from_code(cartridge_type) {
			Some(t) => t,
			None  	=> panic!("Unknown cartridge type: {:#X}", cartridge_type)
		};
		println!("The cartridge type is {:?}", cartridge_type);

		self.mem.cart = Mbc::new(cartridge_type);
		self.mem.cart.load_game(&mut game_file);
		self.mem.save_file = to_save(path);

		if let Ok(mut file) = File::open(self.mem.save_file.clone()) {
			println!("Loading .rsav save file from {}", self.mem.save_file);
			self.mem.cart.load_sav(&mut file);
		}

		let rom_size = header[0x148];
		let rom_size = match get_rom_size(rom_size) {
			Some(size) 	=> size * 1024,
			None 		=> panic!("Unkown ROM size type: {}", rom_size)
		};
		println!("{} has {} bytes ({} KB) of ROM", title, rom_size, rom_size/1024);

		let ram_size = header[0x149];
		let ram_size = match get_ram_size(ram_size) {
			Some(size)	=> size * 1024,
			None		=> panic!("Unknown RAM size type: {}", ram_size)
		};
		println!("{} has {} bytes ({} KB) of external RAM", title, ram_size, ram_size/1024);

		println!("Successfully loaded {}\n", title);

		self.cgb_mode = if self.mem.bios.len() == 0 && header[0x143] & 0x80 == 0 {
			*self.regs.a() = 0x01;
			false
		} else {
			self.mem.bios.len() != 0x100
		};
		self.mem.cgb_mode = self.cgb_mode;
		println!("Emulator running in {}CGB mode", if self.cgb_mode {""} else {"Non-"});
	}
	pub(in emulator) fn enable_interrupts(&mut self) {
		self.interrupts.ime = true;
	}
	pub(in emulator) fn disable_interrupts(&mut self) {
		self.interrupts.ime = false;
	}
	pub fn update_keys(&mut self, key: u8, pressed: bool) {
		let old_state = self.mem.rb(0xFF00);
		for i in 0..8 {
			if self.controls[i] == key {
				self.mem.wk(i as u8, pressed);
				let new_state = self.mem.rb(0xFF00);
				if (!new_state & old_state & (1 << i%4)) > 0 {
					self.interrupts.request_interrupt(&mut self.mem, 4);
				}
			}
		}
	}
	pub fn step(&mut self, state: &mut ProgramState) -> u64 {
		let cycles = if !self.halted && !self.stopped {self.emulate_cycle(state)} else {40};
		self.gpu.step(&mut self.mem, &self.interrupts, cycles as i16, self.cgb_mode);
		self.timers.step(&mut self.mem, &self.interrupts, cycles as i16);
		self.mem.cart.step(cycles as i16);
		if self.interrupts.step(&mut self.mem, &mut self.regs) {
			self.halted = false;
		}

		if self.regs.pc == 0x100 {
			self.mem.finished_with_bios();
		}
		cycles
	}
	pub fn save_game(&mut self) -> usize {
		if let Ok(mut file) = File::create(self.mem.save_file.clone()) {
			self.mem.cart.save_game(&mut file)
		} else {
			0
		}
	}
	// Needs some cleaning up
	// Doesn't produce perfectly correct output, and is messy code
	pub fn disassemble_file(file: &str) -> String {
		if let Ok(mut file) = File::open(file) {
			let mut disassembly = String::new();

			let mut data = Vec::new();
			let _ = file.read_to_end(&mut data);

			// Assume there is a NOP followed by a JP at address 0x100
			let start = data[0x102] as usize | ((data[0x103] as usize) << 8);

			// Also add interrupt handlers
			let mut stack = vec![start, 0x40, 0x48, 0x50, 0x58, 0x60];
			let mut visited: HashSet<_> = stack.iter().cloned().collect();

			while let Some(mut index) = stack.pop() {
				let mut instruction = INSTRUCTIONS[0];
				while !instruction.is_ret() && index < data.len() - 2 {
					visited.insert(index);

					instruction = INSTRUCTIONS[data[index] as usize];
					let step = instruction.operand_length + 1;

					let bytes = [data[index], data[index+1], data[index+2]];
					disassembly = disassembly + &Emulator::disassemble(index as u16, bytes) + "\n";

					if instruction.is_call() || instruction.is_jump() {
						let addr = bytes[1] as usize | ((bytes[2] as usize) << 8);
						if !visited.contains(&addr) {
							stack.push(addr);
							// Too many inserts in this function? Probably but meh
							visited.insert(index);

							if instruction.is_jump() {
								break;
							}
						}
					}
					index += step as usize;
				}
				disassembly += "\n";
			}
			disassembly
		} else {
			String::new()
		}
	}
	pub fn get_screen(&self) -> &[[super::Color; 160]; 144] {
		self.gpu.get_screen()
	}
	pub fn rb(&self, addr: u16) -> u8 {
		self.mem.rb(addr)
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

		if opcode == 0x20 && operand == 0xFE && !self.regs.get_flag(ZERO_FLAG) {
			// jump back 2 bytes if zero flag not set
			// program counter will return to pointing to this instruction and then repeat
			panic!("Error: Emulation caught in infinite loop");
		}

		let cycles: u64;
		if let Some(func) = instruction.func {
			if state.debug {
				if state.debug_regs {
					println!("{:?}", self.regs);
				}
				println!("{}\n", Emulator::disassemble(address, [self.mem.rb(address), self.mem.rb(address+1), self.mem.rb(address+2)]));
			} 

			cycles = func(self, operand);
		} else {
			println!("\nUnimplemented instruction at memory address ({:#X}) [{:#X} ({} | {})] called with operand {:#X}\n", 
				address, opcode, instruction.name, instruction.operand_length, operand);
			panic!("");
		}
		
		self.clock += cycles;
		cycles
	}
	fn disassemble(address: u16, bytes: [u8; 3]) -> String {
		const OP_TYPES: [&'static str; 5] = ["d16", "a8", "a16", "r8", "d8"];

		let opcode = bytes[0];	
		let instruction = INSTRUCTIONS[opcode as usize];
		let disassemble_op = |i: usize| { if i < instruction.operand_length as usize
			{format!("{:#X}", bytes[i+1])} else {"    ".to_string()}
		};

		let operand = if instruction.operand_length == 1 {
			bytes[1] as u16
		} else {
			bytes[1] as u16 | ((bytes[2] as u16) << 8)
		};
		let mut disassembly = instruction.name.to_string();

		// First time I've ever wanted a C style for loop in Rust
		let mut i = 0;
		while disassembly == instruction.name && i < 5 {
			disassembly = disassembly.replace(OP_TYPES[i], &format!("{:#X}", operand));
			i += 1;
		}
		format!("{:#X}:\t{:#X} {} {}\t{}", address, opcode, disassemble_op(0), disassemble_op(1), disassembly)
	}
}
