use std::fmt;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;

use emulator::memory::Memory;
use emulator::gpu::Gpu;
use emulator::interrupts::InterruptManager;
use emulator::timers::Timers;
use emulator::mbc::*;
use emulator::instructions::*;
use emulator::registers::*;
use emulator::rom_info::*;

use super::super::programstate::*;

pub struct Emulator {
	clock:			u64,
	interrupts:		InterruptManager,
	controls: 		[u8; 8],
	timers:			Timers,

	pub mem:		Memory,
	pub gpu:		Gpu,
	pub regs:		Registers,
	pub halted:		bool,
	pub stopped:	bool
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
		Emulator{clock: 0, mem: Memory::new(), gpu: Gpu::new(), controls: [0; 8], 
					regs: Registers::new(), halted: false, timers: Timers::new(),
					interrupts: InterruptManager::new(), stopped: false}
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
					*self.regs.af() = 0x11B0; //CGB mode - 0x01B0 for GB
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
		let mut game_file = File::open(path).unwrap();
		
		let mut header = [0; 0x150];
		let _ = game_file.read(&mut header).unwrap();
		let _ = game_file.seek(SeekFrom::Start(0));

		let title = String::from_utf8_lossy(&header[0x134..0x144]);
		println!("The title of the game is {}", title);
		
		let cartridge_type = header[0x147];
		let cartridge_type = match CartridgeType::from_code(cartridge_type) {
			Some(t) => t,
			None  	=> panic!("Unknown cartridge type: {:#X}", cartridge_type)
		};
		println!("The cartridge type is {:?}", cartridge_type);

		self.mem.cart = Mbc::new(cartridge_type);
		self.mem.cart.load_game(&mut game_file);

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
	}
	pub fn enable_interrupts(&mut self) {
		self.interrupts.ime = true;
	}
	pub fn disable_interrupts(&mut self) {
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
		self.gpu.step(&mut self.mem, &self.interrupts, cycles as i16);
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
			if state.debug {
				println!("Running instruction {:#X} ({} | {}) with operand {:#X} at address ({:#X})\n{:?}\n",
							opcode, instruction.name, instruction.operand_length, operand, address, self);
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
}