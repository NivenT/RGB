use emulator::mbc::Mbc;

pub struct Memory {
	pub cart:		Mbc,
	pub bios:		Vec<u8>, //Size depends on GB/GBC
	
	mem:			[u8; 0x10000],
	key_state:		u8,
	running_bios:	bool
}

impl Memory {
	pub fn new() -> Memory {
		Memory{mem: [0; 0x10000], bios: Vec::new(), cart: Mbc::EMPTY, 
				key_state: 0xFF, running_bios: true}
	}
	pub fn finished_with_bios(&mut self) {
		self.running_bios = false;
	}
	//read byte
	pub fn rb(&self, address: u16) -> u8 {
		let address = address as usize;
		if address < self.bios.len() {
			if self.running_bios {self.bios[address]} else {self.cart.rb(address)}
		} else if address < 0x8000 {
			self.cart.rb(address)
		} else if 0xA000 <= address && address < 0xC000 {
			self.cart.rb(address)
		} else if 0xFF00 == address {
			match self.mem[0xFF00] & 0x30 {
				0x10 => 0x10 | (self.key_state >> 4),
				0x20 => 0x20 | (self.key_state & 0xF),
				_ => 0
			}
		} else {
			self.mem[address]
		}
	}
	//read word
	pub fn rw(&self, address: u16) -> u16 {
		self.rb(address) as u16 | ((self.rb(address+1) as u16) << 8)
	}
	//write byte
	pub fn wb(&mut self, address: u16, val: u8) {
		let address = address as usize;
		if 0xFEA0 <= address && address < 0xFF00 {
			return;
		} else if address < 0x8000 {
			return self.cart.wb(address, val);
		} else if 0xA000 <= address && address < 0xC000 {
			return self.cart.wb(address, val);
		} else if 0xC000 <= address && address < 0xDE00 {
			self.mem[address + 0x2000] = val;
		} else if 0xE000 <= address && address < 0xFE00 {
			self.mem[address - 0x2000] = val;
		} else if 0xFF44 == address {
			panic!("Attempted to overwrite scanline position")
		} else if 0xFF46 == address {
			let start = (val as u16) << 8;
			for i in 0..0xA0 {
				let copy_val = self.rb(start + i);
				self.wb(0xFE00 + i, copy_val);
			}
			return;
		} else if 0xFF04 == address {
			self.mem[0xFF04] = 0;
		}
		self.mem[address] = val;
	}
	//write word
	pub fn ww(&mut self, address: u16, val: u16) {
		self.wb(address, (val & 0x00FF) as u8);
		self.wb(address+1, ((val & 0xFF00) >> 8) as u8)
	}
	//write line (sets the current scanline)
	pub fn wl(&mut self, val: u8) {
		self.mem[0xFF44] = val;
	}
	//write keys
	pub fn wk(&mut self, key: u8, pressed: bool) {
		if pressed {
			self.key_state &= !(1 << key);
		} else {
			self.key_state |= 1 << key;
		}
	}
}