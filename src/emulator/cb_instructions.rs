use emulator::emulator::Emulator;
use emulator::registers::*;

macro_rules! new_cb_instruction {
    ($name:expr, $func:expr) => {
    	CBInstruction{name: $name, func: $func}
    }
}

macro_rules! bit {
	($shift:expr, hl) => {
    	|emu| {
    		unsafe {
    			let val = emu.mem.rb(*emu.regs.hl());
    			emu.regs.update_flags(ZERO_FLAG, ((1 << $shift) & val) == 0);
    		}
    		emu.regs.clear_flags(NEGATIVE_FLAG);
    		emu.regs.set_flags(HALFCARRY_FLAG);
    		16
    	}
    };

    ($shift:expr, $reg:ident) => {
    	|emu| {
    		let val = *emu.regs.$reg();
    		emu.regs.update_flags(ZERO_FLAG, ((1 << $shift) & val) == 0);
    		emu.regs.clear_flags(NEGATIVE_FLAG);
    		emu.regs.set_flags(HALFCARRY_FLAG);
    		8
    	}
    }
}

macro_rules! set {
    ($shift:expr, hl) => {
    	|emu| {
    		unsafe {
    			let val = emu.mem.rb(*emu.regs.hl());
    			emu.mem.wb(*emu.regs.hl(), val | (1 << $shift));
    		}
    		16
    	}
    };

    ($shift:expr, $reg:ident) => {
    	|emu| {
    		*emu.regs.$reg() |= 1 << $shift;
    		8
    	}
    }
}

macro_rules! rl {
	(hl) => {
		|emu| {
			unsafe {
				let carry = if emu.regs.get_flag(CARRY_FLAG) {1} else {0};
				let val = emu.mem.rb(*emu.regs.hl());
    			emu.regs.update_flags(CARRY_FLAG, (val & 0x80) > 0);

    			emu.mem.wb(*emu.regs.hl(), (val << 1) | carry);

    			let val = emu.mem.rb(*emu.regs.hl());
    			emu.regs.update_flags(ZERO_FLAG, val == 0);
    			emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
    			16
			}
		}
	};

    ($reg:ident) => {
    	|emu| {
    		let carry = if emu.regs.get_flag(CARRY_FLAG) {1} else {0};
    		let val = *emu.regs.$reg();
    		emu.regs.update_flags(CARRY_FLAG, (val & 0x80) > 0);

    		*emu.regs.$reg() = (val << 1) | carry;

    		let val = *emu.regs.$reg();
    		emu.regs.update_flags(ZERO_FLAG, val == 0);
    		emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
    		8
    	}
    }
}

macro_rules! res {
    ($shift:expr, hl) => {
    	|emu| {
    		unsafe {
    			let val = emu.mem.rb(*emu.regs.hl());
    			emu.mem.wb(*emu.regs.hl(), val & !(1 << $shift));
    		}
    		16
    	}
    };

    ($shift:expr, $reg:ident) => {
    	|emu| {
    		*emu.regs.$reg() &= !(1 << $shift);
    		8
    	}
    }
}

macro_rules! swap {
	(hl) => {
		|emu| {
			unsafe {
				let val = emu.mem.rb(*emu.regs.hl());
				emu.mem.wb(*emu.regs.hl(), ((val & 0x0F) << 4) | ((val & 0xF0) >> 4));

				emu.regs.update_flags(ZERO_FLAG, val == 0);
    			emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG | CARRY_FLAG);
    			16
			}
		}
	};

    ($reg:ident) => {
    	|emu| {
    		let val = *emu.regs.$reg();
    		*emu.regs.$reg() = (val << 4) | (val >> 4);

    		emu.regs.update_flags(ZERO_FLAG, val == 0);
    		emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG | CARRY_FLAG);
    		8
    	}
    }
}

macro_rules! sla {
    (hl) => {
    	|emu| {
    		unsafe {
	    		let val = emu.mem.rb(*emu.regs.hl());
		    	let carry = (val & 0x80) > 0;
		    	emu.mem.wb(*emu.regs.hl(), val << 1);

		    	emu.regs.update_flags(ZERO_FLAG, val == 0);
		    	emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
		    	emu.regs.update_flags(CARRY_FLAG, carry);
		    	16
	    	}
    	}
    };

    ($reg:ident) => {
    	|emu| {
    		let val = *emu.regs.$reg();
	    	let carry = (val & 0x80) > 0;
	    	*emu.regs.$reg() <<= 1;

	    	emu.regs.update_flags(ZERO_FLAG, val == 0);
	    	emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
	    	emu.regs.update_flags(CARRY_FLAG, carry);
	    	8
    	}
    }
}

macro_rules! srl {
    (hl) => {
    	|emu| {
    		unsafe {
	    		let val = emu.mem.rb(*emu.regs.hl());
		    	let carry = (val & 0x01) > 0;
		    	emu.mem.wb(*emu.regs.hl(), val >> 1);

		    	emu.regs.update_flags(ZERO_FLAG, val == 0);
		    	emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
		    	emu.regs.update_flags(CARRY_FLAG, carry);
		    	16
	    	}
    	}
    };

    ($reg:ident) => {
    	|emu| {
    		let val = *emu.regs.$reg();
	    	let carry = (val & 0x01) > 0;
	    	*emu.regs.$reg() >>= 1;

	    	emu.regs.update_flags(ZERO_FLAG, val == 0);
	    	emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
	    	emu.regs.update_flags(CARRY_FLAG, carry);
	    	8
    	}
    }
}

macro_rules! rlc {
    (hl) => {
    	|emu| {
    		unsafe {
	    		let val = emu.mem.rb(*emu.regs.hl());
	    		let carry = (val & 0x80) >> 7;
	    		emu.mem.wb(*emu.regs.hl(), (val << 1) | carry);

	    		emu.regs.update_flags(ZERO_FLAG, val == 0);
	    		emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
	    		emu.regs.update_flags(CARRY_FLAG, carry > 0);
	    		16
    		}
    	}
    };

    ($reg:ident) => {
    	|emu| {
    		let val = *emu.regs.$reg();
    		let carry = (val & 0x80) >> 7;
    		*emu.regs.$reg() = (val << 1) | carry;

    		emu.regs.update_flags(ZERO_FLAG, val == 0);
    		emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
    		emu.regs.update_flags(CARRY_FLAG, carry > 0);
    		8
    	}
    };
}

macro_rules! rrc {
    (hl) => {
    	|emu| {
    		unsafe {
    			let val = emu.mem.rb(*emu.regs.hl());
	    		let carry = val & 0x01;
	    		emu.mem.wb(*emu.regs.hl(), (val >> 1) | (carry << 7));

	    		emu.regs.update_flags(ZERO_FLAG, val == 0);
	    		emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
	    		emu.regs.update_flags(CARRY_FLAG, carry > 0);
	    		16
    		}
    	}
    };

    ($reg:ident) => {
    	|emu| {
    		let val = *emu.regs.$reg();
    		let carry = val & 0x01;
    		*emu.regs.$reg() = (val >> 1) | (carry << 7);

    		emu.regs.update_flags(ZERO_FLAG, val == 0);
    		emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
    		emu.regs.update_flags(CARRY_FLAG, carry > 0);
    		8
    	}
    }
}

macro_rules! rr {
    (hl) => {
    	|emu| {
    		unsafe {
    			let val = emu.mem.rb(*emu.regs.hl());
	    		let carry = emu.regs.get_flag(CARRY_FLAG) as u8;
	    		emu.mem.wb(*emu.regs.hl(), (val >> 1) | (carry << 7));

	    		emu.regs.update_flags(ZERO_FLAG, val < 2 && carry == 0);
	    		emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
	    		emu.regs.update_flags(CARRY_FLAG, val & 0x01 > 0);
	    		16
    		}
    	}
    };

    ($reg:ident) => {
    	|emu| {
    		let val = *emu.regs.$reg();
    		let carry = emu.regs.get_flag(CARRY_FLAG) as u8;
    		*emu.regs.$reg() = (val >> 1) | (carry << 7);

    		emu.regs.update_flags(ZERO_FLAG, val < 2 && carry == 0);
    		emu.regs.clear_flags(NEGATIVE_FLAG | HALFCARRY_FLAG);
    		emu.regs.update_flags(CARRY_FLAG, val & 0x01 > 0);
    		8
    	}
    }
}

pub type CBInstructionFunc = Option<&'static Fn(&mut Emulator) -> u64>;

#[derive(Copy, Clone)]
pub struct CBInstruction {
	pub name:			&'static str,
	pub func:			CBInstructionFunc
}

pub const CB_INSTRUCTIONS: [CBInstruction; 256] = [
	//0x00
	new_cb_instruction!("RLC B", Some(&rlc!(b))),
	new_cb_instruction!("RLC C", Some(&rlc!(c))),
	new_cb_instruction!("RLC D", Some(&rlc!(d))),
	new_cb_instruction!("RLC E", Some(&rlc!(e))),
	new_cb_instruction!("RLC H", Some(&rlc!(h))),
	new_cb_instruction!("RLC L", Some(&rlc!(l))),
	new_cb_instruction!("RLC (HL)", Some(&rlc!(hl))),
	new_cb_instruction!("RLC A", Some(&rlc!(a))),
	//0x08
	new_cb_instruction!("RRC B", Some(&rrc!(b))),
	new_cb_instruction!("RRC C", Some(&rrc!(c))),
	new_cb_instruction!("RRC D", Some(&rrc!(d))),
	new_cb_instruction!("RRC E", Some(&rrc!(e))),
	new_cb_instruction!("RRC H", Some(&rrc!(h))),
	new_cb_instruction!("RRC L", Some(&rrc!(l))),
	new_cb_instruction!("RRC (HL)", Some(&rrc!(hl))),
	new_cb_instruction!("RRC A", Some(&rrc!(a))),
	//0x10
	new_cb_instruction!("RL B", Some(&rl!(b))),
	new_cb_instruction!("RL C", Some(&rl!(c))),
	new_cb_instruction!("RL D", Some(&rl!(d))),
	new_cb_instruction!("RL E", Some(&rl!(e))),
	new_cb_instruction!("RL H", Some(&rl!(h))),
	new_cb_instruction!("RL L", Some(&rl!(l))),
	new_cb_instruction!("RL (HL)", Some(&rl!(hl))),
	new_cb_instruction!("RL A", Some(&rl!(a))),
	//0x18
	new_cb_instruction!("RR B", Some(&rr!(b))),
	new_cb_instruction!("RR C", Some(&rr!(c))),
	new_cb_instruction!("RR D", Some(&rr!(d))),
	new_cb_instruction!("RR E", Some(&rr!(e))),
	new_cb_instruction!("RR H", Some(&rr!(h))),
	new_cb_instruction!("RR L", Some(&rr!(l))),
	new_cb_instruction!("RR (HL)", Some(&rr!(hl))),
	new_cb_instruction!("RR A", Some(&rr!(a))),
	//0x20
	new_cb_instruction!("SLA B", Some(&sla!(b))),
	new_cb_instruction!("SLA C", Some(&sla!(c))),
	new_cb_instruction!("SLA D", Some(&sla!(d))),
	new_cb_instruction!("SLA E", Some(&sla!(e))),
	new_cb_instruction!("SLA H", Some(&sla!(h))),
	new_cb_instruction!("SLA L", Some(&sla!(l))),
	new_cb_instruction!("SLA (HL)", Some(&sla!(hl))),
	new_cb_instruction!("SLA A", Some(&sla!(a))),
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
	new_cb_instruction!("SWAP B", Some(&swap!(b))),
	new_cb_instruction!("SWAP C", Some(&swap!(c))),
	new_cb_instruction!("SWAP D", Some(&swap!(d))),
	new_cb_instruction!("SWAP E", Some(&swap!(e))),
	new_cb_instruction!("SWAP H", Some(&swap!(h))),
	new_cb_instruction!("SWAP L", Some(&swap!(l))),
	new_cb_instruction!("SWAP (HL)", Some(&swap!(hl))),
	new_cb_instruction!("SWAP A", Some(&swap!(a))),
	//0x38
	new_cb_instruction!("SRL B", Some(&srl!(b))),
	new_cb_instruction!("SRL C", Some(&srl!(c))),
	new_cb_instruction!("SRL D", Some(&srl!(d))),
	new_cb_instruction!("SRL E", Some(&srl!(e))),
	new_cb_instruction!("SRL H", Some(&srl!(h))),
	new_cb_instruction!("SRL L", Some(&srl!(l))),
	new_cb_instruction!("SRL (HL)", Some(&srl!(hl))),
	new_cb_instruction!("SRL A", Some(&srl!(a))),
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
	new_cb_instruction!("RES 0,B", Some(&res!(0, b))),
	new_cb_instruction!("RES 0,C", Some(&res!(0, c))),
	new_cb_instruction!("RES 0,D", Some(&res!(0, d))),
	new_cb_instruction!("RES 0,E", Some(&res!(0, e))),
	new_cb_instruction!("RES 0,H", Some(&res!(0, h))),
	new_cb_instruction!("RES 0,L", Some(&res!(0, l))),
	new_cb_instruction!("RES 0,(HL)", Some(&res!(0, hl))),
	new_cb_instruction!("RES 0,A", Some(&res!(0, a))),
	//0x88
	new_cb_instruction!("RES 1,B", Some(&res!(1, b))),
	new_cb_instruction!("RES 1,C", Some(&res!(1, c))),
	new_cb_instruction!("RES 1,D", Some(&res!(1, d))),
	new_cb_instruction!("RES 1,E", Some(&res!(1, e))),
	new_cb_instruction!("RES 1,H", Some(&res!(1, h))),
	new_cb_instruction!("RES 1,L", Some(&res!(1, l))),
	new_cb_instruction!("RES 1,(HL)", Some(&res!(1, hl))),
	new_cb_instruction!("RES 1,A", Some(&res!(1, a))),
	//0x90
	new_cb_instruction!("RES 2,B", Some(&res!(2, b))),
	new_cb_instruction!("RES 2,C", Some(&res!(2, c))),
	new_cb_instruction!("RES 2,D", Some(&res!(2, d))),
	new_cb_instruction!("RES 2,E", Some(&res!(2, e))),
	new_cb_instruction!("RES 2,H", Some(&res!(2, h))),
	new_cb_instruction!("RES 2,L", Some(&res!(2, l))),
	new_cb_instruction!("RES 2,(HL)", Some(&res!(2, hl))),
	new_cb_instruction!("RES 2,A", Some(&res!(2, a))),
	//0x98
	new_cb_instruction!("RES 3,B", Some(&res!(3, b))),
	new_cb_instruction!("RES 3,C", Some(&res!(3, c))),
	new_cb_instruction!("RES 3,D", Some(&res!(3, d))),
	new_cb_instruction!("RES 3,E", Some(&res!(3, e))),
	new_cb_instruction!("RES 3,H", Some(&res!(3, h))),
	new_cb_instruction!("RES 3,L", Some(&res!(3, l))),
	new_cb_instruction!("RES 3,(HL)", Some(&res!(3, hl))),
	new_cb_instruction!("RES 3,A", Some(&res!(3, a))),
	//0xA0
	new_cb_instruction!("RES 4,B", Some(&res!(4, b))),
	new_cb_instruction!("RES 4,C", Some(&res!(4, c))),
	new_cb_instruction!("RES 4,D", Some(&res!(4, d))),
	new_cb_instruction!("RES 4,E", Some(&res!(4, e))),
	new_cb_instruction!("RES 4,H", Some(&res!(4, h))),
	new_cb_instruction!("RES 4,L", Some(&res!(4, l))),
	new_cb_instruction!("RES 4,(HL)", Some(&res!(4, hl))),
	new_cb_instruction!("RES 4,A", Some(&res!(4, a))),
	//0xA8
	new_cb_instruction!("RES 5,B", Some(&res!(5, b))),
	new_cb_instruction!("RES 5,C", Some(&res!(5, c))),
	new_cb_instruction!("RES 5,D", Some(&res!(5, d))),
	new_cb_instruction!("RES 5,E", Some(&res!(5, e))),
	new_cb_instruction!("RES 5,H", Some(&res!(5, h))),
	new_cb_instruction!("RES 5,L", Some(&res!(5, l))),
	new_cb_instruction!("RES 5,(HL)", Some(&res!(5, hl))),
	new_cb_instruction!("RES 5,A", Some(&res!(5, a))),
	//0xB0
	new_cb_instruction!("RES 6,B", Some(&res!(6, b))),
	new_cb_instruction!("RES 6,C", Some(&res!(6, c))),
	new_cb_instruction!("RES 6,D", Some(&res!(6, d))),
	new_cb_instruction!("RES 6,E", Some(&res!(6, e))),
	new_cb_instruction!("RES 6,H", Some(&res!(6, h))),
	new_cb_instruction!("RES 6,L", Some(&res!(6, l))),
	new_cb_instruction!("RES 6,(HL)", Some(&res!(6, hl))),
	new_cb_instruction!("RES 6,A", Some(&res!(6, a))),
	//0xB8
	new_cb_instruction!("RES 7,B", Some(&res!(7, b))),
	new_cb_instruction!("RES 7,C", Some(&res!(7, c))),
	new_cb_instruction!("RES 7,D", Some(&res!(7, d))),
	new_cb_instruction!("RES 7,E", Some(&res!(7, e))),
	new_cb_instruction!("RES 7,H", Some(&res!(7, h))),
	new_cb_instruction!("RES 7,L", Some(&res!(7, l))),
	new_cb_instruction!("RES 7,(HL)", Some(&res!(7, hl))),
	new_cb_instruction!("RES 7,A", Some(&res!(7, a))),
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
			*emu.regs.hl() = 0xFFB5;
			assert_eq!(*emu.regs.hl(), 0xFFB5);
		}
		assert_eq!(*emu.regs.h(), 0xFF);
		assert_eq!(*emu.regs.l(), 0xB5);
		let orig = emu.mem.rb(0xFFB5);
		let set_3_hl = CB_INSTRUCTIONS[0xDE].func.unwrap();
		set_3_hl(&mut emu);
		assert_eq!(emu.mem.rb(0xFFB5), orig | 8);
	}
	#[test]
	fn test_rl() {
		let mut emu = Emulator::new();
		*emu.regs.a() = 23;
		*emu.regs.f() = CARRY_FLAG;
		let rl_a = CB_INSTRUCTIONS[0x17].func.unwrap();
		rl_a(&mut emu);
		assert_eq!(*emu.regs.a(), 47);
		assert_eq!(*emu.regs.f(), 0);
		*emu.regs.a() = 0x80;
		rl_a(&mut emu);
		assert_eq!(*emu.regs.a(), 0);
		assert_eq!(*emu.regs.f(), ZERO_FLAG | CARRY_FLAG);
	}
	#[test]
	fn test_res() {
		let mut emu = Emulator::new();
		*emu.regs.c() = 0x18;
		let res_4_c = CB_INSTRUCTIONS[0xA1].func.unwrap();
		res_4_c(&mut emu);
		assert_eq!(*emu.regs.c(), 0x08);
	}
	#[test]
	fn test_swap() {
		let mut emu = Emulator::new();
		*emu.regs.l() = 0xFA;
		let swap_l = CB_INSTRUCTIONS[0x35].func.unwrap();
		swap_l(&mut emu);
		assert_eq!(*emu.regs.l(), 0xAF);
	}
	#[test]
	fn test_sla() {
		let mut emu = Emulator::new();
		*emu.regs.e() = 0xC3;
		let sla_e = CB_INSTRUCTIONS[0x23].func.unwrap();
		sla_e(&mut emu);
		assert_eq!(*emu.regs.e(), 0x86);
		assert_eq!(*emu.regs.f(), CARRY_FLAG);
	}
	#[test]
	fn test_srl() {
		let mut emu = Emulator::new();
		*emu.regs.a() = 0x10;
		let srl_a = CB_INSTRUCTIONS[0x3F].func.unwrap();
		srl_a(&mut emu);
		assert_eq!(*emu.regs.a(), 0x08);
		assert_eq!(*emu.regs.f(), 0);
	}
	#[test]
	fn test_rlc() {
		let mut emu = Emulator::new();
		unsafe {
			*emu.regs.hl() = 0xFF1A;
			emu.mem.wb(0xFF1A, 0x7A);
			let rlc_hl = CB_INSTRUCTIONS[0x06].func.unwrap();
			rlc_hl(&mut emu);
			assert_eq!(*emu.regs.hl(), 0xFF1A);
			assert_eq!(emu.mem.rb(0xFF1A), 0xF4);
			assert_eq!(*emu.regs.f(), 0);
		}
	}
	#[test]
	fn test_rrc() {
		let mut emu = Emulator::new();
		*emu.regs.d() = 0x8F;
		let rrc_d = CB_INSTRUCTIONS[0x0A].func.unwrap();
		rrc_d(&mut emu);
		assert_eq!(*emu.regs.d(), 0xC7);
		assert_eq!(*emu.regs.f(), CARRY_FLAG);
	}
	#[test]
	fn test_rr() {
		let mut emu = Emulator::new();
		*emu.regs.b() = 0x01;
		let rr_b = CB_INSTRUCTIONS[0x18].func.unwrap();
		rr_b(&mut emu);
		assert_eq!(*emu.regs.b(), 0);
		assert_eq!(*emu.regs.f(), ZERO_FLAG | CARRY_FLAG);
	}
}