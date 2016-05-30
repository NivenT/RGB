/** Gameboy's 8-bit registers
		In order, A F B C D E H L **/
#[derive(Debug, Default)]
pub struct Registers {
	mem: [u8; 8],
	pc:  u16,
	sp:  u16
}

#[allow(dead_code)]
impl Registers {
	pub fn new() -> Registers {
	    Registers{mem: [0; 8], pc: 0, sp: 0}
	}
	pub fn a(&mut self) -> &mut u8 {
		&mut self.mem[0]
	}
	pub fn f(&mut self) -> &mut u8 {
		&mut self.mem[1]
	}
	pub fn b(&mut self) -> &mut u8 {
		&mut self.mem[2]
	}
	pub fn c(&mut self) -> &mut u8 {
		&mut self.mem[3]
	}
	pub fn d(&mut self) -> &mut u8 {
		&mut self.mem[4]
	}
	pub fn e(&mut self) -> &mut u8 {
		&mut self.mem[5]
	}
	pub fn h(&mut self) -> &mut u8 {
		&mut self.mem[6]
	}
	pub fn l(&mut self) -> &mut u8 {
		&mut self.mem[7]
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
}

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
		assert_eq!(*reg.a(), 0);
		assert_eq!(*reg.f(), 1);
	}
}