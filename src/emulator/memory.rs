/* This level of detail may not be needed
pub struct Memory {
	rom_bank0:		[u8; 0x4000], //0x0000-0x3FFF
	rom_bankn:		[u8; 0x4000], //0x4000-0x7FFF
	vram:			[u8; 0x2000], //0x8000-0x9FFF
	external_ram:	[u8; 0x2000], //0xA000-0xBFFF
	wram_bank0:		[u8; 0x1000], //0xC000-0xCFFF
	//switchiable bank 1-7
	wram_bankn:		[u8; 0x1000], //0xD000-0xDFFF
	//Same data as 0xC000-0xDDFF (wram)
	echo:			[u8; 0x1E00], //0xE000-0xFDFF
	//Sprite Attribute Table
	oam:			[u8; 0x00A0], //0xFE00-0xFE9F
								  //0xFEA0-0xFEFF is not used
	io:				[u8; 0x0080], //0xFF00-0xFF7F
	hram:			[u8; 0x0079], //0xFF7F-0xFFFE
	//Interrupt Enable Register
	ier:			u8			  //0xFFFF
}
*/

//Memory struct exists in case memory management needs to become more complicated
pub struct Memory {
	pub cart:			[u8; 0x08000], //Largest possible cartridge size is 4096 KiB. Only 32 KiB supported right now
	
	rom:				[u8; 0x10000],
	running_bios:		bool
}

impl Memory {
	pub fn new() -> Memory {
	    Memory{rom: [0; 0x10000], cart: [0; 0x08000], running_bios: true}
	}
	//read byte
	pub fn rb(&self, address: u16) -> u8 {
		let address = address as usize;
		if address < 0x4000 {
			if self.running_bios {self.rom[address]} else {self.cart[address]}
		} else if 0x4000 <= address && address < 0x8000 {
			//ROM banking - Not Implemented
			self.rom[address]
		} else if 0xA000 <= address && address < 0xC000 {
			//External RAM banking - Not Implemented
			self.rom[address]
		} else if 0xD000 <= address && address < 0xE000 {
			//Work RAM banking - Not Implemented
			self.rom[address]
		} else {
			self.rom[address]
		}
	}
	//read word
	pub fn rw(&self, address: u16) -> u16 {
		self.rb(address) as u16 + (self.rb(address+1) as u16) << 8
	}
	//write byte
	pub fn wb(&mut self, address: u16, val: u8) {
		let address = address as usize;
		if 0xFEA0 <= address && address < 0xFF00 {
			return panic!("Attempted to write data to unaddressable memory");
		} else if 0xE000 <= address && address < 0xFE00 {
			self.rom[address - 0x2000] = val;
		}
		self.rom[address] = val;
	}
	//write word
	pub fn ww(&mut self, address: u16, val: u16) {
		self.wb(address, (val & 0x00FF) as u8);
		self.wb(address+1, ((val & 0xFF00) >> 8) as u8)
	}
}