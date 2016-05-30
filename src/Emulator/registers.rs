/** Gameboy's 8-bit registers
		In order, A F B C D E H L **/
#[derive(Debug)]
pub struct Registers {
	mem: [u8; 8]
}

#[allow(dead_code)]
impl Registers {
	pub fn new() -> Registers {
	    Registers{mem: [0; 8]}
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
}