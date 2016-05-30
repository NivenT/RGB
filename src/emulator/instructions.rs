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

pub const INSTRUCTIONS: [Instruction; 17] = [
	new_instruction!("NOP", 0, Some(&|_| ())),		//0x00
	new_instruction!("LD BC, 0x%04X", 2, None),
	new_instruction!("LD (BC), A", 0, None),
	new_instruction!("INC BC", 0, None),
	new_instruction!("INC B", 0, None),
	new_instruction!("DEC B", 0, None),
	new_instruction!("LD B", 1, None),
	new_instruction!("RLCA", 0, None),
	new_instruction!("LD (a16),SP", 2, None),		//0x08
	new_instruction!("ADD HL,BC", 0, None),
	new_instruction!("LD A,(BC)", 0, None),
	new_instruction!("DEC BC", 0, None),
	new_instruction!("INC C", 0, None),
	new_instruction!("DEC C", 0, None),
	new_instruction!("LD C", 1, None),
	new_instruction!("RRCA", 0, None),
	new_instruction!("STOP 0", 1, None),			//0x10
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