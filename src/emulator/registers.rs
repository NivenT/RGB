/** Gameboy's 8-bit registers
		In order, A F B C D E H L **/
#[derive(Debug, Default)]
pub struct Registers {
	mem: 	[u8; 8],
	
	pub pc:	u16,
	pub sp: u16
}

#[allow(dead_code)]
impl Registers {
	pub fn new() -> Registers {
	    Registers{mem: [0; 8], pc: 0, sp: 0}
	}
	//Register access
	pub fn a(&mut self) -> &mut u8 {
		&mut self.mem[1]
	}
	pub fn f(&mut self) -> &mut u8 {
		&mut self.mem[0]
	}
	pub fn b(&mut self) -> &mut u8 {
		&mut self.mem[3]
	}
	pub fn c(&mut self) -> &mut u8 {
		&mut self.mem[2]
	}
	pub fn d(&mut self) -> &mut u8 {
		&mut self.mem[5]
	}
	pub fn e(&mut self) -> &mut u8 {
		&mut self.mem[4]
	}
	pub fn h(&mut self) -> &mut u8 {
		&mut self.mem[7]
	}
	pub fn l(&mut self) -> &mut u8 {
		&mut self.mem[6]
	}
	pub fn af(&mut self) -> *mut u16 {
		&mut self.mem[..2] as *mut _ as *mut u16
	}
	pub fn bc(&mut self) -> *mut u16 {
		&mut self.mem[2..4] as *mut _ as *mut u16
	}
	pub fn de(&mut self) -> *mut u16 {
		&mut self.mem[4..6] as *mut _ as *mut u16
	}
	pub fn hl(&mut self) -> *mut u16 {
		&mut self.mem[6..] as *mut _ as *mut u16
	}
	//Flag manipulation
	pub fn set_flags(&mut self, mask: u8) {
		self.mem[0] |= mask;
	}
	pub fn clear_flags(&mut self, mask: u8) {
		self.mem[0] &= !mask;
	}
	pub fn update_flags(&mut self, mask: u8, val: bool) {
		if val {
			self.set_flags(mask)
		} else {
			self.clear_flags(mask)
		}
	}
	pub fn get_flag(&self, ident: u8) -> bool {
		(self.mem[0] & ident) > 0
	}
}

pub const ZERO_FLAG: u8 		= 0x80;
pub const NEGATIVE_FLAG: u8 	= 0x40;
pub const HALFCARRY_FLAG: u8 	= 0x20;
pub const CARRY_FLAG: u8 		= 0x10;

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_ind_reg() {
		let mut reg = Registers::new();
		assert_eq!(*reg.a(), 0);
		*reg.a() = 5;
		assert_eq!(*reg.a(), 5);
	}

	#[test]
	fn test_joint_reg() {
		let mut reg = Registers::new();
		assert_eq!(*reg.a(), 0);
		unsafe{*reg.af() = 256;}
		assert_eq!(*reg.a(), 1);
		assert_eq!(*reg.f(), 0);
	}
}