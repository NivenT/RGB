pub const BIOS: [u8; 0x100] = [
	0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
	0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
	0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
	0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
	0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
	0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
	0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
	0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
	0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
	0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
	0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
	0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
	0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
	0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
	0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
	0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50
];

pub struct Memory {
	pub cart:		[u8; 0x08000], //Largest possible cartridge size is 4096 KiB. Only 32 KiB supported right now
	
	mem:			[u8; 0x10000],
	running_bios:	bool
}

impl Memory {
	pub fn new() -> Memory {
	    Memory{mem: [0; 0x10000], cart: [0; 0x08000], running_bios: true}
	}
	pub fn finished_with_bios(&mut self) {
		self.running_bios = false;
	}
	//read byte
	pub fn rb(&self, address: u16) -> u8 {
		let address = address as usize;
		if address < 0x0100 {
			if self.running_bios {self.mem[address]} else {self.cart[address]}
		} else if address < 0x4000 {
			self.cart[address]
		} else if 0x4000 <= address && address < 0x8000 {
			//ROM banking - Not Implemented
			self.cart[address]
		} else if 0xA000 <= address && address < 0xC000 {
			//External RAM banking - Not Implemented
			self.cart[address]
		} else if 0xD000 <= address && address < 0xE000 {
			//Work RAM banking - Not Implemented
			self.mem[address]
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
		} else if 0xC000 <= address && address < 0xDE00 {
			self.mem[address + 0x2000] = val;
		} else if 0xE000 <= address && address < 0xFE00 {
			self.mem[address - 0x2000] = val;
		} else if 0xFF44 == address {
			panic!("Attempted to overwrite scanline position")
		} else if 0xFF00 == address {
			self.mem[address] = (val & 0x30) | (self.mem[address] & 0xF);
			return;
		} else if 0xFF46 == address {
			let start = (val as u16) << 8;
			for i in 0..0xA0 {
				let copy_val = self.rb(start + i);
				self.wb(0xFE00 + i, copy_val);
			}
			return;
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
	pub fn wk(&mut self, val: u8) {
		self.mem[0xFF00] = (val & 0xF) | (self.mem[0xFF00] & 0x30);
	}
}