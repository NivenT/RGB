use emulator::registers::Registers;

//Instruction constructor macro because consts can't call functions
macro_rules! new_instruction {
    ($name:expr, $operand_length:expr, $func:expr) => {
    	Instruction{name: $name, operand_length: $operand_length, func: $func}
    }
}

type InstructionFunc = Option<&'static Fn(&mut Registers) -> ()>;

#[derive(Copy, Clone)]
pub struct Instruction {
	name:			&'static str,
	operand_length:	u8,
	func:			InstructionFunc
}

#[allow(dead_code)]
impl Instruction {
	pub fn call(self, regs: &mut Registers) -> bool{
		match self.func {
			Some(f) => {f(regs); true},
			None => false
		}
	}
}

pub const INSTRUCTIONS: [Instruction; 3] = [
	new_instruction!("NOP", 0, Some(&|_| ())),
	new_instruction!("LD BC, 0x%04X", 2, None),
	new_instruction!("LD (BC), A", 0, None),
];

#[cfg(test)]
mod test {
	use super::*;
	use emulator::registers::Registers;

	#[test]
	fn test_instruction_call() {
		let mut regs = Registers::new();
		INSTRUCTIONS[0].call(&mut regs);
	}
}