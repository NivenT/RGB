use emulator::emulator::Emulator;
use emulator::registers::*;

macro_rules! new_cb_instruction {
    ($name:expr, $func:expr) => {
    	CBInstruction{name: $name, func: $func}
    }
}

macro_rules! bit {
	($shift:expr, hl) => {
    	|emu: &mut Emulator| {
    		unsafe {
    			if ((1 << $shift) & emu.memory[*emu.regs.hl() as usize]) > 0 {
    				emu.regs.clear_flags(ZERO_FLAG);
    			} else {
    				emu.regs.set_flags(ZERO_FLAG);
    			}
    		}

    		emu.regs.clear_flags(NEGATIVE_FLAG);
    		emu.regs.set_flags(HALFCARRY_FLAG);
    	}
    };

    ($shift:expr, $reg:ident) => {
    	|emu: &mut Emulator| {
    		if ((1 << $shift) & *emu.regs.$reg()) > 0 {
    			emu.regs.clear_flags(ZERO_FLAG);
    		} else {
    			emu.regs.set_flags(ZERO_FLAG);
    		}

    		emu.regs.clear_flags(NEGATIVE_FLAG);
    		emu.regs.set_flags(HALFCARRY_FLAG);
    	}
    }
}

macro_rules! set {
    ($shift:expr, hl) => {
    	|emu: &mut Emulator| {
    		unsafe {
    			emu.memory[*emu.regs.hl() as usize] |= 1 << $shift;
    		}
    	}
    };

    ($shift:expr, $reg:ident) => {
    	|emu: &mut Emulator| {
    		*emu.regs.$reg() |= 1 << $shift;
    	}
    }
}

pub type CBInstructionFunc = Option<&'static Fn(&mut Emulator) -> ()>;

#[derive(Copy, Clone)]
pub struct CBInstruction {
	pub name:			&'static str,
	pub func:			CBInstructionFunc
}

pub const CB_INSTRUCTIONS: [CBInstruction; 256] = [
	//0x00
	new_cb_instruction!("RLC B", None),
	new_cb_instruction!("RLC C", None),
	new_cb_instruction!("RLC D", None),
	new_cb_instruction!("RLC E", None),
	new_cb_instruction!("RLC H", None),
	new_cb_instruction!("RLC L", None),
	new_cb_instruction!("RLC (HL)", None),
	new_cb_instruction!("RLC A", None),
	//0x08
	new_cb_instruction!("RRC B", None),
	new_cb_instruction!("RRC C", None),
	new_cb_instruction!("RRC D", None),
	new_cb_instruction!("RRC E", None),
	new_cb_instruction!("RRC H", None),
	new_cb_instruction!("RRC L", None),
	new_cb_instruction!("RRC (HL)", None),
	new_cb_instruction!("RRC A", None),
	//0x10
	new_cb_instruction!("RL B", None),
	new_cb_instruction!("RL C", None),
	new_cb_instruction!("RL D", None),
	new_cb_instruction!("RL E", None),
	new_cb_instruction!("RL H", None),
	new_cb_instruction!("RL L", None),
	new_cb_instruction!("RL (HL)", None),
	new_cb_instruction!("RL A", None),
	//0x18
	new_cb_instruction!("RR B", None),
	new_cb_instruction!("RR C", None),
	new_cb_instruction!("RR D", None),
	new_cb_instruction!("RR E", None),
	new_cb_instruction!("RR H", None),
	new_cb_instruction!("RR L", None),
	new_cb_instruction!("RR (HL)", None),
	new_cb_instruction!("RR A", None),
	//0x20
	new_cb_instruction!("SLA B", None),
	new_cb_instruction!("SLA C", None),
	new_cb_instruction!("SLA D", None),
	new_cb_instruction!("SLA E", None),
	new_cb_instruction!("SLA H", None),
	new_cb_instruction!("SLA L", None),
	new_cb_instruction!("SLA (HL)", None),
	new_cb_instruction!("SLA A", None),
	//0x28
	new_cb_instruction!("SRA B", None),
	new_cb_instruction!("SRA C", None),
	new_cb_instruction!("SRA D", None),
	new_cb_instruction!("SRA E", None),
	new_cb_instruction!("SRA H", None),
	new_cb_instruction!("SRA L", None),
	new_cb_instruction!("SRA (HL)", None),
	new_cb_instruction!("SRA A", None),
	//0x30
	new_cb_instruction!("SWAP B", None),
	new_cb_instruction!("SWAP C", None),
	new_cb_instruction!("SWAP D", None),
	new_cb_instruction!("SWAP E", None),
	new_cb_instruction!("SWAP H", None),
	new_cb_instruction!("SWAP L", None),
	new_cb_instruction!("SWAP (HL)", None),
	new_cb_instruction!("SWAP A", None),
	//0x38
	new_cb_instruction!("SRL B", None),
	new_cb_instruction!("SRL C", None),
	new_cb_instruction!("SRL D", None),
	new_cb_instruction!("SRL E", None),
	new_cb_instruction!("SRL H", None),
	new_cb_instruction!("SRL L", None),
	new_cb_instruction!("SRL (HL)", None),
	new_cb_instruction!("SRL A", None),
	//0x40
	new_cb_instruction!("BIT 0,B", Some(&bit!(0, b))),
	new_cb_instruction!("BIT 0,C", Some(&bit!(0, c))),
	new_cb_instruction!("BIT 0,D", Some(&bit!(0, d))),
	new_cb_instruction!("BIT 0,E", Some(&bit!(0, e))),
	new_cb_instruction!("BIT 0,H", Some(&bit!(0, h))),
	new_cb_instruction!("BIT 0,L", Some(&bit!(0, l))),
	new_cb_instruction!("BIT 0,(HL)", Some(&bit!(0, hl))),
	new_cb_instruction!("BIT 0,A", Some(&bit!(0, a))),
	//0x48
	new_cb_instruction!("BIT 1,B", Some(&bit!(1, b))),
	new_cb_instruction!("BIT 1,C", Some(&bit!(1, c))),
	new_cb_instruction!("BIT 1,D", Some(&bit!(1, d))),
	new_cb_instruction!("BIT 1,E", Some(&bit!(1, e))),
	new_cb_instruction!("BIT 1,H", Some(&bit!(1, h))),
	new_cb_instruction!("BIT 1,L", Some(&bit!(1, l))),
	new_cb_instruction!("BIT 1,(HL)", Some(&bit!(1, hl))),
	new_cb_instruction!("BIT 1,A", Some(&bit!(1, a))),
	//0x50
	new_cb_instruction!("BIT 2,B", Some(&bit!(2, b))),
	new_cb_instruction!("BIT 2,C", Some(&bit!(2, c))),
	new_cb_instruction!("BIT 2,D", Some(&bit!(2, d))),
	new_cb_instruction!("BIT 2,E", Some(&bit!(2, e))),
	new_cb_instruction!("BIT 2,H", Some(&bit!(2, h))),
	new_cb_instruction!("BIT 2,L", Some(&bit!(2, l))),
	new_cb_instruction!("BIT 2,(HL)", Some(&bit!(2, hl))),
	new_cb_instruction!("BIT 2,A", Some(&bit!(2, a))),
	//0x58
	new_cb_instruction!("BIT 3,B", Some(&bit!(3, b))),
	new_cb_instruction!("BIT 3,C", Some(&bit!(3, c))),
	new_cb_instruction!("BIT 3,D", Some(&bit!(3, d))),
	new_cb_instruction!("BIT 3,E", Some(&bit!(3, e))),
	new_cb_instruction!("BIT 3,H", Some(&bit!(3, h))),
	new_cb_instruction!("BIT 3,L", Some(&bit!(3, l))),
	new_cb_instruction!("BIT 3,(HL)", Some(&bit!(3, hl))),
	new_cb_instruction!("BIT 3,A", Some(&bit!(3, a))),
	//0x60
	new_cb_instruction!("BIT 4,B", Some(&bit!(4, b))),
	new_cb_instruction!("BIT 4,C", Some(&bit!(4, c))),
	new_cb_instruction!("BIT 4,D", Some(&bit!(4, d))),
	new_cb_instruction!("BIT 4,E", Some(&bit!(4, e))),
	new_cb_instruction!("BIT 4,H", Some(&bit!(4, h))),
	new_cb_instruction!("BIT 4,L", Some(&bit!(4, l))),
	new_cb_instruction!("BIT 4,(HL)", Some(&bit!(4, hl))),
	new_cb_instruction!("BIT 4,A", Some(&bit!(4, a))),
	//0x68
	new_cb_instruction!("BIT 5,B", Some(&bit!(5, b))),
	new_cb_instruction!("BIT 5,C", Some(&bit!(5, c))),
	new_cb_instruction!("BIT 5,D", Some(&bit!(5, d))),
	new_cb_instruction!("BIT 5,E", Some(&bit!(5, e))),
	new_cb_instruction!("BIT 5,H", Some(&bit!(5, h))),
	new_cb_instruction!("BIT 5,L", Some(&bit!(5, l))),
	new_cb_instruction!("BIT 5,(HL)", Some(&bit!(5, hl))),
	new_cb_instruction!("BIT 5,A", Some(&bit!(5, a))),
	//0x70
	new_cb_instruction!("BIT 6,B", Some(&bit!(6, b))),
	new_cb_instruction!("BIT 6,C", Some(&bit!(6, c))),
	new_cb_instruction!("BIT 6,D", Some(&bit!(6, d))),
	new_cb_instruction!("BIT 6,E", Some(&bit!(6, e))),
	new_cb_instruction!("BIT 6,H", Some(&bit!(6, h))),
	new_cb_instruction!("BIT 6,L", Some(&bit!(6, l))),
	new_cb_instruction!("BIT 6,(HL)", Some(&bit!(6, hl))),
	new_cb_instruction!("BIT 6,A", Some(&bit!(6, a))),
	//0x78
	new_cb_instruction!("BIT 7,B", Some(&bit!(7, b))),
	new_cb_instruction!("BIT 7,C", Some(&bit!(7, c))),
	new_cb_instruction!("BIT 7,D", Some(&bit!(7, d))),
	new_cb_instruction!("BIT 7,E", Some(&bit!(7, e))),
	new_cb_instruction!("BIT 7,H", Some(&bit!(7, h))),
	new_cb_instruction!("BIT 7,L", Some(&bit!(7, l))),
	new_cb_instruction!("BIT 7,(HL)", Some(&bit!(7, hl))),
	new_cb_instruction!("BIT 7,A", Some(&bit!(7, a))),
	//0x80
	new_cb_instruction!("RES 0,B", None),
	new_cb_instruction!("RES 0,C", None),
	new_cb_instruction!("RES 0,D", None),
	new_cb_instruction!("RES 0,E", None),
	new_cb_instruction!("RES 0,H", None),
	new_cb_instruction!("RES 0,L", None),
	new_cb_instruction!("RES 0,(HL)", None),
	new_cb_instruction!("RES 0,A", None),
	//0x88
	new_cb_instruction!("RES 1,B", None),
	new_cb_instruction!("RES 1,C", None),
	new_cb_instruction!("RES 1,D", None),
	new_cb_instruction!("RES 1,E", None),
	new_cb_instruction!("RES 1,H", None),
	new_cb_instruction!("RES 1,L", None),
	new_cb_instruction!("RES 1,(HL)", None),
	new_cb_instruction!("RES 1,A", None),
	//0x90
	new_cb_instruction!("RES 2,B", None),
	new_cb_instruction!("RES 2,C", None),
	new_cb_instruction!("RES 2,D", None),
	new_cb_instruction!("RES 2,E", None),
	new_cb_instruction!("RES 2,H", None),
	new_cb_instruction!("RES 2,L", None),
	new_cb_instruction!("RES 2,(HL)", None),
	new_cb_instruction!("RES 2,A", None),
	//0x98
	new_cb_instruction!("RES 3,B", None),
	new_cb_instruction!("RES 3,C", None),
	new_cb_instruction!("RES 3,D", None),
	new_cb_instruction!("RES 3,E", None),
	new_cb_instruction!("RES 3,H", None),
	new_cb_instruction!("RES 3,L", None),
	new_cb_instruction!("RES 3,(HL)", None),
	new_cb_instruction!("RES 3,A", None),
	//0xA0
	new_cb_instruction!("RES 4,B", None),
	new_cb_instruction!("RES 4,C", None),
	new_cb_instruction!("RES 4,D", None),
	new_cb_instruction!("RES 4,E", None),
	new_cb_instruction!("RES 4,H", None),
	new_cb_instruction!("RES 4,L", None),
	new_cb_instruction!("RES 4,(HL)", None),
	new_cb_instruction!("RES 4,A", None),
	//0xA8
	new_cb_instruction!("RES 5,B", None),
	new_cb_instruction!("RES 5,C", None),
	new_cb_instruction!("RES 5,D", None),
	new_cb_instruction!("RES 5,E", None),
	new_cb_instruction!("RES 5,H", None),
	new_cb_instruction!("RES 5,L", None),
	new_cb_instruction!("RES 5,(HL)", None),
	new_cb_instruction!("RES 5,A", None),
	//0xB0
	new_cb_instruction!("RES 6,B", None),
	new_cb_instruction!("RES 6,C", None),
	new_cb_instruction!("RES 6,D", None),
	new_cb_instruction!("RES 6,E", None),
	new_cb_instruction!("RES 6,H", None),
	new_cb_instruction!("RES 6,L", None),
	new_cb_instruction!("RES 6,(HL)", None),
	new_cb_instruction!("RES 6,A", None),
	//0xB8
	new_cb_instruction!("RES 7,B", None),
	new_cb_instruction!("RES 7,C", None),
	new_cb_instruction!("RES 7,D", None),
	new_cb_instruction!("RES 7,E", None),
	new_cb_instruction!("RES 7,H", None),
	new_cb_instruction!("RES 7,L", None),
	new_cb_instruction!("RES 7,(HL)", None),
	new_cb_instruction!("RES 7,A", None),
	//0xC0
	new_cb_instruction!("SET 0,B", Some(&set!(0, b))),
	new_cb_instruction!("SET 0,C", Some(&set!(0, c))),
	new_cb_instruction!("SET 0,D", Some(&set!(0, d))),
	new_cb_instruction!("SET 0,E", Some(&set!(0, e))),
	new_cb_instruction!("SET 0,H", Some(&set!(0, h))),
	new_cb_instruction!("SET 0,L", Some(&set!(0, l))),
	new_cb_instruction!("SET 0,(HL)", Some(&set!(0, hl))),
	new_cb_instruction!("SET 0,A", Some(&set!(0, a))),
	//0xC8
	new_cb_instruction!("SET 1,B", Some(&set!(1, b))),
	new_cb_instruction!("SET 1,C", Some(&set!(1, c))),
	new_cb_instruction!("SET 1,D", Some(&set!(1, d))),
	new_cb_instruction!("SET 1,E", Some(&set!(1, e))),
	new_cb_instruction!("SET 1,H", Some(&set!(1, h))),
	new_cb_instruction!("SET 1,L", Some(&set!(1, l))),
	new_cb_instruction!("SET 1,(HL)", Some(&set!(1, hl))),
	new_cb_instruction!("SET 1,A", Some(&set!(1, a))),
	//0xD0
	new_cb_instruction!("SET 2,B", Some(&set!(2, b))),
	new_cb_instruction!("SET 2,C", Some(&set!(2, c))),
	new_cb_instruction!("SET 2,D", Some(&set!(2, d))),
	new_cb_instruction!("SET 2,E", Some(&set!(2, e))),
	new_cb_instruction!("SET 2,H", Some(&set!(2, h))),
	new_cb_instruction!("SET 2,L", Some(&set!(2, l))),
	new_cb_instruction!("SET 2,(HL)", Some(&set!(2, hl))),
	new_cb_instruction!("SET 2,A", Some(&set!(2, a))),
	//0xD8
	new_cb_instruction!("SET 3,B", Some(&set!(3, b))),
	new_cb_instruction!("SET 3,C", Some(&set!(3, c))),
	new_cb_instruction!("SET 3,D", Some(&set!(3, d))),
	new_cb_instruction!("SET 3,E", Some(&set!(3, e))),
	new_cb_instruction!("SET 3,H", Some(&set!(3, h))),
	new_cb_instruction!("SET 3,L", Some(&set!(3, l))),
	new_cb_instruction!("SET 3,(HL)", Some(&set!(3, hl))),
	new_cb_instruction!("SET 3,A", Some(&set!(3, a))),
	//0xE0
	new_cb_instruction!("SET 4,B", Some(&set!(4, b))),
	new_cb_instruction!("SET 4,C", Some(&set!(4, c))),
	new_cb_instruction!("SET 4,D", Some(&set!(4, d))),
	new_cb_instruction!("SET 4,E", Some(&set!(4, e))),
	new_cb_instruction!("SET 4,H", Some(&set!(4, h))),
	new_cb_instruction!("SET 4,L", Some(&set!(4, l))),
	new_cb_instruction!("SET 4,(HL)", Some(&set!(4, hl))),
	new_cb_instruction!("SET 4,A", Some(&set!(4, a))),
	//0xE8
	new_cb_instruction!("SET 5,B", Some(&set!(5, b))),
	new_cb_instruction!("SET 5,C", Some(&set!(5, c))),
	new_cb_instruction!("SET 5,D", Some(&set!(5, d))),
	new_cb_instruction!("SET 5,E", Some(&set!(5, e))),
	new_cb_instruction!("SET 5,H", Some(&set!(5, h))),
	new_cb_instruction!("SET 5,L", Some(&set!(5, l))),
	new_cb_instruction!("SET 5,(HL)", Some(&set!(5, hl))),
	new_cb_instruction!("SET 5,A", Some(&set!(5, a))),
	//0xF0
	new_cb_instruction!("SET 6,B", Some(&set!(6, b))),
	new_cb_instruction!("SET 6,C", Some(&set!(6, c))),
	new_cb_instruction!("SET 6,D", Some(&set!(6, d))),
	new_cb_instruction!("SET 6,E", Some(&set!(6, e))),
	new_cb_instruction!("SET 6,H", Some(&set!(6, h))),
	new_cb_instruction!("SET 6,L", Some(&set!(6, l))),
	new_cb_instruction!("SET 6,(HL)", Some(&set!(6, hl))),
	new_cb_instruction!("SET 6,A", Some(&set!(6, a))),
	//0xF8
	new_cb_instruction!("SET 7,B", Some(&set!(7, b))),
	new_cb_instruction!("SET 7,C", Some(&set!(7, c))),
	new_cb_instruction!("SET 7,D", Some(&set!(7, d))),
	new_cb_instruction!("SET 7,E", Some(&set!(7, e))),
	new_cb_instruction!("SET 7,H", Some(&set!(7, h))),
	new_cb_instruction!("SET 7,L", Some(&set!(7, l))),
	new_cb_instruction!("SET 7,(HL)", Some(&set!(7, hl))),
	new_cb_instruction!("SET 7,A",Some(&set!(7, a)))
];

#[cfg(test)]
mod test {
	use super::*;
	use emulator::registers::*;
	use emulator::emulator::Emulator;

	#[test]
	fn test_bit() {
		let mut emu = Emulator::new();
		*emu.regs.a() = 100;
		assert_eq!(*emu.regs.a(), 100);
		assert_eq!(*emu.regs.f(), 0);
		let bit_7_a = CB_INSTRUCTIONS[0x7F].func.unwrap();
		bit_7_a(&mut emu);
		assert_eq!(*emu.regs.a(), 100);
		assert_eq!(*emu.regs.f(), HALFCARRY_FLAG | ZERO_FLAG);
		let bit_5_a = CB_INSTRUCTIONS[0x6F].func.unwrap();
		bit_5_a(&mut emu);
		assert_eq!(*emu.regs.a(), 100);
		assert_eq!(*emu.regs.f(), HALFCARRY_FLAG);
	}

	#[test]
	fn test_set() {
		let mut emu = Emulator::new();
		unsafe{
			*emu.regs.hl() = 2049;
			assert_eq!(*emu.regs.hl(), 2049);
		}
		assert_eq!(*emu.regs.h(), 8);
		assert_eq!(*emu.regs.l(), 1);
		let orig = emu.memory[2049];
		let set_3_hl = CB_INSTRUCTIONS[0xDE].func.unwrap();
		set_3_hl(&mut emu);
		assert_eq!(emu.memory[2049], orig | 8);
	}
}