use emulator::mbc::Mbc;

pub struct Memory {
	pub cart:		Mbc,
	pub bios:		Vec<u8>, 	//Size depends on GB/GBC
	
	mem:			Vec<u8>, 	//64 KB
	wram:			Vec<u8>, 	//32 KB (8 4KB banks)
	vram:			Vec<u8>, 	//16 KB (2 8KB banks)
	bgp:			[u8; 64], 	//Background Palette Memory
	wram_bank:		u8,
	key_state:		u8,
	running_bios:	bool
}

impl Memory {
	pub fn new() -> Memory {
		Memory {
			mem: vec![0; 0x10000], 
			wram: vec![0; 0x8000], 
			vram: vec![0; 0x4000],
			bgp: [0; 64], 
			bios: Vec::new(), 
			cart: Mbc::EMPTY, 
			wram_bank: 1, 
			key_state: 0xFF, 
			running_bios: true
		}
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
		} else if 0x8000 <= address && address < 0xA000 {
			self.vram[self.rb(0xFF4F) as usize*0x2000 + address%0x8000]
		} else if 0xA000 <= address && address < 0xC000 {
			self.cart.rb(address)
		} else if 0xC000 <= address && address < 0xD000 {
			self.wram[address - 0xC000]
		} else if 0xD000 <= address && address < 0xE000 {
			self.wram[self.wram_bank as usize*0x1000 + address%0xD000]
		} else if 0xFF00 == address {
			match self.mem[0xFF00] & 0x30 {
				0x10 => 0x10 | (self.key_state >> 4),
				0x20 => 0x20 | (self.key_state & 0xF),
				_ => 0
			}
		} else if 0xFF69 == address {
			self.bgp[(self.rb(0xFF68) & 0x3F) as usize]
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
		} else if 0x8000 <= address && address < 0xA000 {
			let bank = self.rb(0xFF4F);
			self.vram[bank as usize*0x2000 + address%0x8000] = val;
		} else if 0xA000 <= address && address < 0xC000 {
			return self.cart.wb(address, val);
		} else if 0xC000 <= address && address < 0xD000 {
			self.wram[address - 0xC000] = val;
			self.mem[address + 0x2000] = val;
		} else if 0xD000 <= address && address < 0xE000 {
			self.wram[self.wram_bank as usize*0x1000 + address%0xD000] = val;
			if address < 0xDE00 {
				self.mem[address + 0x2000] = val;
			}
		} else if 0xE000 <= address && address < 0xFE00 {
			self.mem[address - 0x2000] = val;
		} else if 0xFF04 == address { //divider register (DIV)
			self.mem[0xFF04] = 0;
		} else if 0xFF44 == address { //scanline position
			panic!("Attempted to overwrite scanline position")
		} else if 0xFF46 == address { //DMA transfer
			let start = (val as u16) << 8;
			for i in 0..0xA0 {
				let copy_val = self.rb(start + i);
				self.wb(0xFE00 + i, copy_val);
			}
			return;
		} else if 0xFF4F == address {
			return self.mem[address] = val & 1;
		} else if 0xFF69 == address { //Background Palette Data
			self.bgp[(self.rb(0xFF68) & 0x3F) as usize] = val;
			if (self.rb(0xFF68) >> 7) > 0 {
				let old_val = self.rb(0xFF68);
				self.wb(0xFF68, (old_val + 1) | (1 << 7));
			}
		} else if 0xFF70 == address { //select wram bank
			self.wram_bank = if (val & 7) == 0 {1} else {val & 7};
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
	pub fn read_vram0(&mut self, address: u16) -> u8 {
		self.vram[(address%0x8000) as usize]
	}
	pub fn read_vram1(&mut self, address: u16) -> u8 {
		self.vram[(0x2000 + address%0x8000) as usize]
	}
	pub fn read_bgpn(&mut self, n: usize) -> u8 {
		self.bgp[n]
	}
}