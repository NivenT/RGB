use emulator::emulator::Emulator;
use emulator::cb_instructions::*;
use emulator::registers::*;

//Instruction constructor macro because consts can't call functions
macro_rules! new_instruction {
    ($name:expr, $operand_length:expr, $func:expr) => {
    	Instruction{name: $name, operand_length: $operand_length, func: $func}
    }
}

macro_rules! xor {
    ($reg:ident) => {
    	|emu, _| {
    		*emu.regs.a() ^= *emu.regs.$reg();

    		emu.regs.clear_flags(ALL_FLAGS);
    		if *emu.regs.a() == 0 {emu.regs.set_flags(ZERO_FLAG);}
    		4

    	}
    }
}

macro_rules! ld {
	(sp, 16) => {
		|emu, operand| {
			emu.regs.sp = operand;
			12
		}
	};

	($reg:ident, 16) => {
    	|emu, operand| {
    		unsafe{*emu.regs.$reg() = operand;}
    		12
    	}
    };

    ($reg:ident, 8) => {
    	|emu, operand| {
    		*emu.regs.$reg() = operand as u8;
    		8
    	}
    };

    (c, mem, $reg:ident) => {
    	|emu, _| {
    		emu.mem.wb(0xFF00 + *emu.regs.c() as u16, *emu.regs.$reg());
    		8
    	}
    };

    ($reg:ident, c, mem) => {
    	|emu, _| {
    		*emu.regs.$reg() = emu.mem.rb(0xFF00 + *emu.regs.c() as u16);
    		8
    	}
    };

    ($reg1:ident, mem, $reg2:ident, $shift:expr) => {
    	|emu, _| {
    		unsafe{
    			emu.mem.wb(*emu.regs.$reg1(), *emu.regs.$reg2());
    			*emu.regs.$reg1() = (*emu.regs.$reg1() as i32 + $shift) as u16;
    		}
    		8
    	}
    };

    ($reg1:ident, $reg2:ident, mem, $shift:expr) => {
    	|emu, _| {
    		unsafe {
    			*emu.regs.$reg1() = emu.mem.rb(*emu.regs.$reg2());
    			*emu.regs.$reg2() = (*emu.regs.$reg2() as i32 + $shift) as u16;
    		}
    		8
    	}
    };

    ($reg1:ident, $reg2:ident) => {
    	|emu, _| {
    		*emu.regs.$reg1() = *emu.regs.$reg2();
    		4
    	}
    }
}

macro_rules! rst {
    ($val:expr) => {
    	|emu, _| {
    		emu.mem.ww(emu.regs.sp-2, emu.regs.pc);
    		emu.regs.pc = $val;
    		emu.regs.sp -= 2;
    		16
    	}
    }
}

macro_rules! inc {
    ($reg:ident, 8) => {
    	|emu, _| {
    		*emu.regs.$reg() += 1;

    		let val = *emu.regs.$reg();
    		emu.regs.update_flags(ZERO_FLAG, val == 0);
    		emu.regs.clear_flags(NEGATIVE_FLAG);
    		emu.regs.update_flags(HALFCARRY_FLAG, (val & 0x1F) == 0x10);
    		4
    	}
    };

    (sp, 16) => {
    	|emu, _| {
    		emu.regs.sp += 1;
    		8
    	}
    };

    ($reg:ident, 16) => {
    	|emu, _| {
    		unsafe {
    			*emu.regs.$reg() += 1;
    			8
    		}
    	}
    };

    ($reg:ident, mem) => {
    	|emu, _| {
    		unsafe {
    			let val = emu.mem.rb(*emu.regs.$reg())+1;
    			emu.mem.wb(*emu.regs.$reg(), val);

    			emu.regs.update_flags(ZERO_FLAG, val == 0);
	    		emu.regs.clear_flags(NEGATIVE_FLAG);
	    		emu.regs.update_flags(HALFCARRY_FLAG, (val & 0x1F) == 0x10);
    			12
    		}
    	}
    }
}

macro_rules! dec {
    ($reg:ident, 8) => {
    	|emu, _| {
    		*emu.regs.$reg() -= 1;

    		let val = *emu.regs.$reg();
    		emu.regs.update_flags(ZERO_FLAG, val == 0);
    		emu.regs.set_flags(NEGATIVE_FLAG);
    		emu.regs.update_flags(HALFCARRY_FLAG, (val & 0xF) != 0xE);
    		4
    	}
    };

    (sp, 16) => {
    	|emu, _| {
    		emu.regs.sp -= 1;
    		8
    	}
    };

    ($reg:ident, 16) => {
    	|emu, _| {
    		unsafe {
    			*emu.regs.$reg() -= 1;
    			8
    		}
    	}
    };

    ($reg:ident, mem) => {
    	|emu, _| {
    		unsafe {
    			let val = emu.mem.rb(*emu.regs.$reg())-1;
    			emu.mem.wb(*emu.regs.$reg(), val);

    			emu.regs.update_flags(ZERO_FLAG, val == 0);
	    		emu.regs.clear_flags(NEGATIVE_FLAG);
	    		emu.regs.update_flags(HALFCARRY_FLAG, (val & 0xF) == 0xE);
    			12
    		}
    	}
    }
}

macro_rules! push {
    ($reg:ident) => {
    	|emu, _| {
    		unsafe {
    			emu.mem.ww(emu.regs.sp-2, *emu.regs.$reg());
    			emu.regs.sp -= 2;
    			16
    		}
    	}
    }
}

macro_rules! pop {
    ($reg:ident) => {
    	|emu, _| {
    		unsafe {
    			*emu.regs.$reg() = emu.mem.rw(emu.regs.sp);
    			emu.regs.sp += 2;
    			12
    		}
    	}
    }
}

macro_rules! cp {
	() => {
		|emu, operand| {
			let (a,b) = (*emu.regs.a() as u16, operand);
	    	emu.regs.update_flags(ZERO_FLAG, a == b);
			emu.regs.set_flags(NEGATIVE_FLAG);
			emu.regs.update_flags(HALFCARRY_FLAG, (a & 0xF) < (b & 0xF));
			emu.regs.update_flags(CARRY_FLAG, a < b);
			8
		}
	};

	(hl) => {
		|emu, _| {
			unsafe {
				let (a,b) = (*emu.regs.a() as u16, *emu.regs.hl());
		    	emu.regs.update_flags(ZERO_FLAG, a == b);
				emu.regs.set_flags(NEGATIVE_FLAG);
				emu.regs.update_flags(HALFCARRY_FLAG, (a & 0xF) < (b & 0xF));
				emu.regs.update_flags(CARRY_FLAG, a < b);
			}
			8
		}
	};

    ($reg:ident) => {
    	|emu, _| {
    		let (a,b) = (*emu.regs.a(), *emu.regs.$reg());
	    	emu.regs.update_flags(ZERO_FLAG, a == b);
			emu.regs.set_flags(NEGATIVE_FLAG);
			emu.regs.update_flags(HALFCARRY_FLAG, (a & 0xF) < (b & 0xF));
			emu.regs.update_flags(CARRY_FLAG, a < b);
    		4    		
    	}
    }
}

//Returns the number of cycles the instruction takes
pub type InstructionFunc = Option<&'static Fn(&mut Emulator, u16) -> u64>;

#[derive(Copy, Clone)]
pub struct Instruction {
	pub name:			&'static str,
	pub operand_length:	u8,
	pub func:			InstructionFunc
}

pub const INSTRUCTIONS: [Instruction; 256] = [
	//0x00
	new_instruction!("NOP", 0, Some(&|_,_| 4)),	
	new_instruction!("LD BC,d16", 2, Some(&ld!(bc, 16))),
	new_instruction!("LD (BC),A", 0, Some(&ld!(bc, mem, a, 0))),
	new_instruction!("INC BC", 0, Some(&inc!(bc, 16))),
	new_instruction!("INC B", 0, Some(&inc!(b, 8))),
	new_instruction!("DEC B", 0, Some(&dec!(b, 8))),
	new_instruction!("LD B,d8", 1, Some(&ld!(b, 8))),
	new_instruction!("RLCA", 0, Some(&rlca)),
	//0x08
	new_instruction!("LD (a16),SP", 2, Some(&ld_a16p_sp)),
	new_instruction!("ADD HL,BC", 0, None),
	new_instruction!("LD A,(BC)", 0, Some(&ld!(a, bc, mem, 0))),
	new_instruction!("DEC BC", 0, Some(&dec!(bc, 16))),
	new_instruction!("INC C", 0, Some(&inc!(c, 8))),
	new_instruction!("DEC C", 0, Some(&dec!(c, 8))),
	new_instruction!("LD C,d8", 1, Some(&ld!(c, 8))),
	new_instruction!("RRCA", 0, Some(&rrca)),
	//0x10
	new_instruction!("STOP 0", 1, None),			
	new_instruction!("LD DE,d16", 2, Some(&ld!(de, 16))),
	new_instruction!("LD (DE),A", 0, Some(&ld!(de, mem, a, 0))),
	new_instruction!("INC DE", 0, Some(&inc!(de, 16))),
	new_instruction!("INC D", 0, Some(&inc!(d, 8))),
	new_instruction!("DEC D", 0, Some(&dec!(d, 8))),
	new_instruction!("LD D,d8", 1, Some(&ld!(d, 8))),
	new_instruction!("RLA", 0, Some(&rla)),
	//0x18
	new_instruction!("JR r8", 1, Some(&jr)),				
	new_instruction!("ADD HL,DE", 0, None),
	new_instruction!("LD A,(DE)", 0, Some(&ld!(a, de, mem, 0))),
	new_instruction!("DEC DE", 0, Some(&dec!(de, 16))),
	new_instruction!("INC E", 0, Some(&inc!(e, 8))),
	new_instruction!("DEC E", 0, Some(&dec!(e, 8))),
	new_instruction!("LD E,d8", 1, Some(&ld!(e, 8))),
	new_instruction!("RRA", 0, Some(&rra)),
	//0x20
	new_instruction!("JR NZ,r8", 1, Some(&jr_nz)),			
	new_instruction!("LD HL,d16", 2, Some(&ld!(hl, 16))),
	new_instruction!("LD (HL+),A", 0, Some(&ld!(hl, mem, a, 1))),
	new_instruction!("INC HL", 0, Some(&inc!(hl, 16))),
	new_instruction!("INC H", 0, Some(&inc!(h, 8))),
	new_instruction!("DEC H", 0, Some(&dec!(h, 8))),
	new_instruction!("LD H,d8", 1, Some(&ld!(h, 8))),
	new_instruction!("DAA", 0, None),
	//0x28
	new_instruction!("JR Z,r8", 1, Some(&jr_z)),			
	new_instruction!("ADD HL,HL", 0, None),
	new_instruction!("LD A,(HL+)", 0, Some(&ld!(a, hl, mem, 1))),
	new_instruction!("DEC HL", 0, Some(&dec!(hl, 16))),
	new_instruction!("INC L", 0, Some(&inc!(l, 8))),
	new_instruction!("DEC L", 0, Some(&dec!(l, 8))),
	new_instruction!("LD L,d8", 1, Some(&ld!(l, 8))),
	new_instruction!("CPL", 0, None),
	//0x30
	new_instruction!("JR NC,r8", 1, Some(&jr_nc)),			
	new_instruction!("LD SP,d16", 2, Some(&ld!(sp, 16))),
	new_instruction!("LD (HL-),A", 0, Some(&ld!(hl, mem, a, -1))),
	new_instruction!("INC SP", 0, Some(&inc!(sp, 16))),
	new_instruction!("INC (HL)", 0, Some(&inc!(hl, mem))),
	new_instruction!("DEC (HL)", 0, Some(&dec!(hl, mem))),
	new_instruction!("LD (HL),d8", 1, Some(&ld_hlp_d8)),
	new_instruction!("SCF", 0, None),
	//0x38
	new_instruction!("JR C,r8", 1, Some(&jr_c)),			
	new_instruction!("ADD HL,SP", 0, None),
	new_instruction!("LD A,(HL-)", 0, Some(&ld!(a, hl, mem, -1))),
	new_instruction!("DEC SP", 0, Some(&dec!(sp, 16))),
	new_instruction!("INC A", 0, Some(&inc!(a, 8))),
	new_instruction!("DEC A", 0, Some(&dec!(a, 8))),
	new_instruction!("LD A,d8", 1, Some(&ld!(a, 8))),
	new_instruction!("CCF", 0, None),
	//0x40
	new_instruction!("LD B,B", 0, Some(&ld!(b, b))),			
	new_instruction!("LD B,C", 0, Some(&ld!(b, c))),
	new_instruction!("LD B,D", 0, Some(&ld!(b, d))),
	new_instruction!("LD B,E", 0, Some(&ld!(b, e))),
	new_instruction!("LD B,H", 0, Some(&ld!(b, h))),
	new_instruction!("LD B,L", 0, Some(&ld!(b, l))),
	new_instruction!("LD B,(HL)", 0, Some(&ld!(b, hl, mem, 0))),
	new_instruction!("LD B,A", 0, Some(&ld!(b, a))),
	//0x48
	new_instruction!("LD C,B", 0, Some(&ld!(c, b))),			
	new_instruction!("LD C,C", 0, Some(&ld!(c, c))),
	new_instruction!("LD C,D", 0, Some(&ld!(c, d))),
	new_instruction!("LD C,E", 0, Some(&ld!(c, e))),
	new_instruction!("LD C,H", 0, Some(&ld!(c, h))),
	new_instruction!("LD C,L", 0, Some(&ld!(c, l))),
	new_instruction!("LD C,(HL)", 0, Some(&ld!(c, hl, mem, 0))),
	new_instruction!("LD C,A", 0, Some(&ld!(c, a))),
	//0x50
	new_instruction!("LD D,B", 0, Some(&ld!(d, b))),			
	new_instruction!("LD D,C", 0, Some(&ld!(d, c))),
	new_instruction!("LD D,D", 0, Some(&ld!(d, d))),
	new_instruction!("LD D,E", 0, Some(&ld!(d, e))),
	new_instruction!("LD D,H", 0, Some(&ld!(d, h))),
	new_instruction!("LD D,L", 0, Some(&ld!(d, l))),
	new_instruction!("LD D,(HL)", 0, Some(&ld!(d, hl, mem, 0))),
	new_instruction!("LD D,A", 0, Some(&ld!(d, a))),
	//0x58
	new_instruction!("LD E,B", 0, Some(&ld!(e, b))),			
	new_instruction!("LD E,C", 0, Some(&ld!(e, c))),
	new_instruction!("LD E,D", 0, Some(&ld!(e, d))),
	new_instruction!("LD E,E", 0, Some(&ld!(e, e))),
	new_instruction!("LD E,H", 0, Some(&ld!(e, h))),
	new_instruction!("LD E,L", 0, Some(&ld!(e, l))),
	new_instruction!("LD E,(HL)", 0, Some(&ld!(e, hl, mem, 0))),
	new_instruction!("LD E,A", 0, Some(&ld!(e, a))),
	//0x60
	new_instruction!("LD H,B", 0, Some(&ld!(h, b))),			
	new_instruction!("LD H,C", 0, Some(&ld!(h, c))),
	new_instruction!("LD H,D", 0, Some(&ld!(h, d))),
	new_instruction!("LD H,E", 0, Some(&ld!(h, e))),
	new_instruction!("LD H,H", 0, Some(&ld!(h, h))),
	new_instruction!("LD H,L", 0, Some(&ld!(h, l))),
	new_instruction!("LD H,(HL)", 0, Some(&ld!(h, hl, mem, 0))),
	new_instruction!("LD H,A", 0, Some(&ld!(h, a))),
	//0x68
	new_instruction!("LD L,B", 0, Some(&ld!(l, b))),			
	new_instruction!("LD L,C", 0, Some(&ld!(l, c))),
	new_instruction!("LD L,D", 0, Some(&ld!(l, d))),
	new_instruction!("LD L,E", 0, Some(&ld!(l, e))),
	new_instruction!("LD L,H", 0, Some(&ld!(l, h))),
	new_instruction!("LD L,L", 0, Some(&ld!(l, l))),
	new_instruction!("LD L,(HL)", 0, Some(&ld!(l, hl, mem, 0))),
	new_instruction!("LD L,A", 0, Some(&ld!(l, a))),
	//0x70
	new_instruction!("LD (HL),B", 0, Some(&ld!(hl, mem, b, 0))),			
	new_instruction!("LD (HL),C", 0, Some(&ld!(hl, mem, c, 0))),
	new_instruction!("LD (HL),D", 0, Some(&ld!(hl, mem, d, 0))),
	new_instruction!("LD (HL),E", 0, Some(&ld!(hl, mem, e, 0))),
	new_instruction!("LD (HL),H", 0, Some(&ld!(hl, mem, h, 0))),
	new_instruction!("LD (HL),L", 0, Some(&ld!(hl, mem, l, 0))),
	new_instruction!("HALT", 0, None),
	new_instruction!("LD (HL),A", 0, Some(&ld!(hl, mem, a, 0))),
	//0x78
	new_instruction!("LD A,B", 0, Some(&ld!(a, b))),			
	new_instruction!("LD A,C", 0, Some(&ld!(a, c))),
	new_instruction!("LD A,D", 0, Some(&ld!(a, d))),
	new_instruction!("LD A,E", 0, Some(&ld!(a, e))),
	new_instruction!("LD A,H", 0, Some(&ld!(a, h))),
	new_instruction!("LD A,L", 0, Some(&ld!(a, l))),
	new_instruction!("LD A,(HL)", 0, Some(&ld!(a, hl, mem, 0))),
	new_instruction!("LD A,A", 0, Some(&ld!(a, a))),
	//0x80
	new_instruction!("ADD A,B", 0, None),			
	new_instruction!("ADD A,C", 0, None),
	new_instruction!("ADD A,D", 0, None),
	new_instruction!("ADD A,E", 0, None),
	new_instruction!("ADD A,H", 0, None),
	new_instruction!("ADD A,L", 0, None),
	new_instruction!("ADD A,(HL)", 0, None),
	new_instruction!("ADD A,A", 0, None),
	//0x88
	new_instruction!("ADC A,B", 0, None),			
	new_instruction!("ADC A,C", 0, None),
	new_instruction!("ADC A,D", 0, None),
	new_instruction!("ADC A,E", 0, None),
	new_instruction!("ADC A,H", 0, None),
	new_instruction!("ADC A,L", 0, None),
	new_instruction!("ADC A,(HL)", 0, None),
	new_instruction!("ADC A,A", 0, None),
	//0x90
	new_instruction!("SUB B", 0, None),				
	new_instruction!("SUB C", 0, None),
	new_instruction!("SUB D", 0, None),
	new_instruction!("SUB E", 0, None),
	new_instruction!("SUB H", 0, None),
	new_instruction!("SUB L", 0, None),
	new_instruction!("SUB (HL)", 0, None),
	new_instruction!("SUB A", 0, None),
	//0x98
	new_instruction!("SBC A,B", 0, None),			
	new_instruction!("SBC A,C", 0, None),
	new_instruction!("SBC A,D", 0, None),
	new_instruction!("SBC A,E", 0, None),
	new_instruction!("SBC A,H", 0, None),
	new_instruction!("SBC A,L", 0, None),
	new_instruction!("SBC A,(HL)", 0, None),
	new_instruction!("SBC A,A", 0, None),
	//0xA0
	new_instruction!("AND B", 0, None),				
	new_instruction!("AND C", 0, None),
	new_instruction!("AND D", 0, None),
	new_instruction!("AND E", 0, None),
	new_instruction!("AND H", 0, None),
	new_instruction!("AND L", 0, None),
	new_instruction!("AND (HL)", 0, None),
	new_instruction!("AND A", 0, None),
	//0xA8
	new_instruction!("XOR B", 0, Some(&xor!(b))),				
	new_instruction!("XOR C", 0, Some(&xor!(c))),
	new_instruction!("XOR D", 0, Some(&xor!(d))),
	new_instruction!("XOR E", 0, Some(&xor!(e))),
	new_instruction!("XOR H", 0, Some(&xor!(h))),
	new_instruction!("XOR L", 0, Some(&xor!(l))),
	new_instruction!("XOR (HL)", 0, None),
	new_instruction!("XOR A", 0, Some(&xor!(a))),
	//0xB0
	new_instruction!("OR B", 0, None),				
	new_instruction!("OR C", 0, None),
	new_instruction!("OR D", 0, None),
	new_instruction!("OR E", 0, None),
	new_instruction!("OR H", 0, None),
	new_instruction!("OR L", 0, None),
	new_instruction!("OR (HL)", 0, None),
	new_instruction!("OR A", 0, None),
	//0xB8
	new_instruction!("CP B", 0, Some(&cp!(b))),				
	new_instruction!("CP C", 0, Some(&cp!(c))),
	new_instruction!("CP D", 0, Some(&cp!(d))),
	new_instruction!("CP E", 0, Some(&cp!(e))),
	new_instruction!("CP H", 0, Some(&cp!(h))),
	new_instruction!("CP L", 0, Some(&cp!(l))),
	new_instruction!("CP (HL)", 0, Some(&cp!(hl))),
	new_instruction!("CP A", 0, Some(&cp!(a))),
	//0xC0
	new_instruction!("RET NZ", 0, Some(&ret_nz)),			
	new_instruction!("POP BC", 0, Some(&pop!(bc))),
	new_instruction!("JP NZ,a16", 2, Some(&jp_nz)),
	new_instruction!("JP a16", 2, Some(&jp)),
	new_instruction!("CALL NZ,a16", 2, Some(&call_nz_a16)),
	new_instruction!("PUSH BC", 0, Some(&push!(bc))),
	new_instruction!("ADD A,d8", 1, None),
	new_instruction!("RST 00H", 0, Some(&rst!(0x0000))),
	//0xC8
	new_instruction!("RET Z", 0, Some(&ret_z)),				
	new_instruction!("RET", 0, Some(&ret)),
	new_instruction!("JP Z,a16", 2, Some(&jp_z)),
	new_instruction!("PREFIX CB", 1, Some(&cb)),
	new_instruction!("CALL Z,a16", 2, Some(&call_z_a16)),
	new_instruction!("CALL a16", 2, Some(&call_a16)),
	new_instruction!("ADC A,d8", 1, None),
	new_instruction!("RST 08H", 0, Some(&rst!(0x0008))),
	//0xD0
	new_instruction!("RET NC", 0, Some(&ret_nc)),			
	new_instruction!("POP DE", 0, Some(&pop!(de))),
	new_instruction!("JP NC,a16", 2, Some(&jp_nc)),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("CALL NC,a16", 2, Some(&call_nc_a16)),
	new_instruction!("PUSH DE", 0, Some(&push!(de))),
	new_instruction!("SUB d8", 1, None),
	new_instruction!("RST 10H", 0, Some(&rst!(0x0010))),
	//0xD8
	new_instruction!("RET C", 0, Some(&ret_c)),				
	new_instruction!("RETI", 0, None),
	new_instruction!("JP C,a16", 2, Some(&jp_c)),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("CALL C,a16", 2, Some(&call_c_a16)),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("SBC A,d8", 1, None),
	new_instruction!("RST 18H", 0, Some(&rst!(0x0018))),
	//0xE0
	new_instruction!("LDH (a8),A", 1, Some(&ldh_a8_a)),		
	new_instruction!("POP HL", 0, Some(&pop!(hl))),
	new_instruction!("LD (C),A", 0, Some(&ld!(c, mem, a))),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("PUSH HL", 0, Some(&push!(hl))),
	new_instruction!("AND d8", 1, None),
	new_instruction!("RST 20H", 0, Some(&rst!(0x0020))),
	//0xE8
	new_instruction!("ADD SP,r8", 1, None),			
	new_instruction!("JP (HL)", 0, Some(&jp_hl)),
	new_instruction!("LD (a16),A", 2, Some(&ld_a16_a)),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("XOR d8", 1, None),
	new_instruction!("RST 28H", 0, Some(&rst!(0x0028))),
	//0xF0
	new_instruction!("LDH A,(a8)", 1, Some(&ldh_a_a8)),		
	new_instruction!("POP AF", 0, Some(&pop!(af))),
	new_instruction!("LD A,(C)", 0, Some(&ld!(a, c, mem))),
	new_instruction!("DI", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("PUSH AF", 0, Some(&push!(af))),
	new_instruction!("OR d8", 1, None),
	new_instruction!("RST 30H", 0, Some(&rst!(0x0030))),
	//0xF8
	new_instruction!("LD HL,SP+r8", 1, Some(&ld_hl_spr8)),	
	new_instruction!("LD SP,HL", 0, Some(&ld_sp_hl)),
	new_instruction!("LD A,(a16)", 2, Some(&ld_a_a16)),
	new_instruction!("EI", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("CP d8", 1, Some(&cp!())),
	new_instruction!("RST 38H", 0, Some(&rst!(0x0038))),
];

//0x70
fn rlca(emu: &mut Emulator, _: u16) -> u64 {
	let carry = *emu.regs.a() & 0x80;
	emu.regs.update_flags(CARRY_FLAG, carry > 0);

	*emu.regs.a() = (*emu.regs.a() << 1) | (carry >> 7);
	emu.regs.clear_flags(ZERO_FLAG | NEGATIVE_FLAG | HALFCARRY_FLAG);
	4
}

//0x08
fn ld_a16p_sp(emu: &mut Emulator, operand: u16) -> u64 {
	emu.mem.ww(operand, emu.regs.sp);
	20
}

//0x0F
fn rrca(emu: &mut Emulator, _: u16) -> u64 {
	let carry = *emu.regs.a() & 0x01;
	emu.regs.update_flags(CARRY_FLAG, carry > 0);

	*emu.regs.a() = (*emu.regs.a() >> 1) | (carry << 7);
	emu.regs.clear_flags(ZERO_FLAG | NEGATIVE_FLAG | HALFCARRY_FLAG);
	4
}

//0x17
fn rla(emu: &mut Emulator, _: u16) -> u64 {
	let carry = if emu.regs.get_flag(CARRY_FLAG) {1} else {0};
	let val = *emu.regs.a();
	emu.regs.update_flags(CARRY_FLAG, (val & 0x80) > 0);

	*emu.regs.a() = (val << 1) | carry;
	emu.regs.clear_flags(ZERO_FLAG | NEGATIVE_FLAG | HALFCARRY_FLAG);
	4
}

//0x18
fn jr(emu: &mut Emulator, operand: u16) -> u64 {
	emu.regs.pc = (emu.regs.pc as i16 + operand as i8 as i16) as u16;
	12
}

//0x1F
fn rra(emu: &mut Emulator, _: u16) -> u64 {
	let carry = if emu.regs.get_flag(CARRY_FLAG) {0x80} else {0};
	let val = *emu.regs.a();
	emu.regs.update_flags(CARRY_FLAG, (val & 0x01) > 0);

	*emu.regs.a() = (val >> 1) | carry;
	emu.regs.clear_flags(ZERO_FLAG | NEGATIVE_FLAG | HALFCARRY_FLAG);
	4
}

//0x20
fn jr_nz(emu: &mut Emulator, operand: u16) -> u64 {
	if !emu.regs.get_flag(ZERO_FLAG) {
		return jr(emu, operand);
	}
	8
}

//0x28
fn jr_z(emu: &mut Emulator, operand: u16) -> u64 {
	if emu.regs.get_flag(ZERO_FLAG) {
		return jr(emu, operand);
	}
	8
}

//0x30
fn jr_nc(emu: &mut Emulator, operand: u16) -> u64 {
	if !emu.regs.get_flag(CARRY_FLAG) {
		return jr(emu, operand);
	}
	8
}

//0x36
fn ld_hlp_d8(emu: &mut Emulator, operand: u16) -> u64 {
	unsafe {
		emu.mem.wb(*emu.regs.hl(), operand as u8);
	}
	12
}

//0x38
fn jr_c(emu: &mut Emulator, operand: u16) -> u64 {
	if emu.regs.get_flag(CARRY_FLAG) {
		return jr(emu, operand);
	}
	8
}

//0xC0
fn ret_nz(emu: &mut Emulator, operand: u16) -> u64 {
	if !emu.regs.get_flag(ZERO_FLAG) {
		return ret(emu, operand)+4;
	}
	8
}

//0xC2
fn jp_nz(emu: &mut Emulator, operand: u16) -> u64 {
	if !emu.regs.get_flag(ZERO_FLAG) {
		return jp(emu, operand);
	}
	12
}

//0xC3
fn jp(emu: &mut Emulator, operand: u16) -> u64 {
	emu.regs.pc = operand;
	16
}

//0xC4
fn call_nz_a16(emu: &mut Emulator, operand: u16) -> u64 {
	if !emu.regs.get_flag(ZERO_FLAG) {
		return call_a16(emu, operand);
	}
	12
}

//0xC8
fn ret_z(emu: &mut Emulator, operand: u16) -> u64 {
	if emu.regs.get_flag(ZERO_FLAG) {
		return ret(emu, operand)+4;
	}
	8
}

//0xC9
fn ret(emu: &mut Emulator, _: u16) -> u64 {
	emu.regs.pc = emu.mem.rw(emu.regs.sp);
	emu.regs.sp += 2;
	16
}

//0xCA
fn jp_z(emu: &mut Emulator, operand: u16) -> u64 {
	if emu.regs.get_flag(ZERO_FLAG) {
		return jp(emu, operand);
	}
	12
}

//0xCB
fn cb(emu: &mut Emulator, operand: u16) -> u64 {
	let instruction = CB_INSTRUCTIONS[operand as usize];
	if let Some(func) = instruction.func {
		return func(emu);
	} else {
		println!("Unimplemented function at memory address ({:#X}) [{:#X} {:#X} ({})]", 
			emu.regs.pc-2, 0xCB, operand, instruction.name);
		panic!("");
	}
}

//0xCC
fn call_z_a16(emu: &mut Emulator, operand: u16) -> u64 {
	if emu.regs.get_flag(ZERO_FLAG) {
		return call_a16(emu, operand);
	}
	12
}

//0xCD
fn call_a16(emu: &mut Emulator, operand: u16) -> u64 {
	emu.mem.ww(emu.regs.sp-2, emu.regs.pc);
	emu.regs.pc = operand;
	emu.regs.sp -= 2;
	24
}

//0xD0
fn ret_nc(emu: &mut Emulator, operand: u16) -> u64 {
	if !emu.regs.get_flag(CARRY_FLAG) {
		return ret(emu, operand)+4;
	}
	8
}

//0xD2
fn jp_nc(emu: &mut Emulator, operand: u16) -> u64 {
	if !emu.regs.get_flag(CARRY_FLAG) {
		return jp(emu, operand);
	}
	12
}

//0xD4
fn call_nc_a16(emu: &mut Emulator, operand: u16) -> u64 {
	if !emu.regs.get_flag(CARRY_FLAG) {
		return call_a16(emu, operand);
	}
	12
}

//0xD8
fn ret_c(emu: &mut Emulator, operand: u16) -> u64 {
	if emu.regs.get_flag(CARRY_FLAG) {
		return ret(emu, operand)+4;
	}
	8
}

//0xDA
fn jp_c(emu: &mut Emulator, operand: u16) -> u64 {
	if emu.regs.get_flag(CARRY_FLAG) {
		return jp(emu, operand);
	}
	12
}

//0xDC
fn call_c_a16(emu: &mut Emulator, operand: u16) -> u64 {
	if emu.regs.get_flag(CARRY_FLAG) {
		return call_a16(emu, operand);
	}
	12
}

//0xE0
fn ldh_a8_a(emu: &mut Emulator, operand: u16) -> u64 {
	emu.mem.wb(0xFF00 + operand, *emu.regs.a());
	12
}

//0xE9
fn jp_hl(emu: &mut Emulator, _: u16) -> u64 {
	unsafe {
		emu.regs.pc = *emu.regs.hl();
	}
	4
}

//0xEA
fn ld_a16_a(emu: &mut Emulator, operand: u16) -> u64 {
	emu.mem.wb(operand, *emu.regs.a());
	16
}

//0xF0
fn ldh_a_a8(emu: &mut Emulator, operand: u16) -> u64 {
	*emu.regs.a() = emu.mem.rb(0xFF00 + operand);
	12
}

//0xF8
fn ld_hl_spr8(emu: &mut Emulator, operand: u16) -> u64 {
	unsafe {
		let result = (emu.regs.sp as i16 + operand as i8 as i16) as u32;
		*emu.regs.hl() = (result & 0xFFFF) as u16;
		
		let val = (operand & 0x0F) + (emu.regs.sp & 0x0F);
		emu.regs.clear_flags(ZERO_FLAG | NEGATIVE_FLAG);
		emu.regs.update_flags(HALFCARRY_FLAG, val > 0x0F);
		emu.regs.update_flags(CARRY_FLAG, result > 0xFFFF);
	}
	12
}

//0xF9
fn ld_sp_hl(emu: &mut Emulator, _: u16) -> u64 {
	unsafe {
		emu.regs.sp = *emu.regs.hl();
	}
	8
}

//0xFA
fn ld_a_a16(emu: &mut Emulator, operand: u16) -> u64 {
	*emu.regs.a() = emu.mem.rb(operand);
	16
}

#[cfg(test)]
mod test {
	use super::*;
	use emulator::registers::*;
	use emulator::emulator::Emulator;
	use emulator::rom_info::BIOS;

	#[test]
	fn test_xor() {
		let mut emu = Emulator::new();
		assert_eq!(*emu.regs.a(), 0);
		assert_eq!(emu.regs.get_flag(ZERO_FLAG), false);
		let xor_a = INSTRUCTIONS[0xAF].func.unwrap();
		xor_a(&mut emu, 0);
		assert_eq!(*emu.regs.a(), 0);
		assert_eq!(emu.regs.get_flag(ZERO_FLAG), true);
	}
	#[test]
	fn test_ld_hld_a() {
		let mut emu = Emulator::new();
		*emu.regs.l() = 0;
		*emu.regs.h() = 255;
		*emu.regs.a() = 18;
		unsafe{
			assert_eq!(*emu.regs.hl(), 65280);
			assert_eq!(emu.mem.rb(5), BIOS[5]);
			let ld_hld_a = INSTRUCTIONS[0x32].func.unwrap();
			ld_hld_a(&mut emu, 0);
			assert_eq!(*emu.regs.hl(), 65279);
			assert_eq!(emu.mem.rb(65280), 18);
		}

	}
	#[test]
	fn test_jr_nz() {
		let mut emu = Emulator::new();
		emu.regs.pc = 1000;
		let jr_nz = INSTRUCTIONS[0x20].func.unwrap();
		jr_nz(&mut emu, 0xEC); //-20 as a signed 8-bit integer
		assert_eq!(emu.regs.pc, 980);
		jr_nz(&mut emu, 0x64); //100 as a signed 8-bit integer
		assert_eq!(emu.regs.pc, 1080);
	}
	#[test]
	fn test_rcca() {
		let mut emu = Emulator::new();
		*emu.regs.a() = 50;
		let rrca = INSTRUCTIONS[0x0F].func.unwrap();
		rrca(&mut emu, 0);
		assert_eq!(*emu.regs.a(), 25);
		assert_eq!(*emu.regs.f(), 0);
		rrca(&mut emu, 0);
		assert_eq!(*emu.regs.a(), 140);
		assert_eq!(*emu.regs.f(), CARRY_FLAG);
	}
	#[test]
	fn test_rst() {
		let mut emu = Emulator::new();
		emu.regs.sp = 3;
		emu.regs.pc = 0xDEAD;
		let rst_20 = INSTRUCTIONS[0x0E7].func.unwrap();
		rst_20(&mut emu, 0);
		assert_eq!(emu.regs.sp, 1);
		assert_eq!(emu.mem.rb(2), 0xDE);
		assert_eq!(emu.mem.rb(1), 0xAD);
		assert_eq!(emu.regs.pc, 0x20);
	}
	#[test]
	fn test_inc_dec() {
		let mut emu = Emulator::new();
		*emu.regs.a() = 15;
		*emu.regs.b() = 15;
		let inc_a = INSTRUCTIONS[0x3C].func.unwrap();
		inc_a(&mut emu, 0);
		assert_eq!(*emu.regs.a(), 16);
		assert_eq!(*emu.regs.f(), HALFCARRY_FLAG);
		let dec_b = INSTRUCTIONS[0x05].func.unwrap();
		dec_b(&mut emu, 0);
		assert_eq!(*emu.regs.b(), 14);
		assert_eq!(*emu.regs.f(), NEGATIVE_FLAG);
	}
	#[test]
	fn test_ld_hl_spr8() {
		let mut emu = Emulator::new();
		emu.regs.sp = 100;
		let ld_hl_spr8 = INSTRUCTIONS[0xF8].func.unwrap();
		ld_hl_spr8(&mut emu, 20);
		unsafe {
			assert_eq!(*emu.regs.hl(), 120);
			assert_eq!(*emu.regs.f(), 0);
		}
		ld_hl_spr8(&mut emu, 0xFF); // -1 as a signed 8-bit integer
		unsafe {
			assert_eq!(*emu.regs.hl(), 99);
			assert_eq!(*emu.regs.f(), HALFCARRY_FLAG);
		}
	}
	#[test]
	fn test_call() {
		let mut emu = Emulator::new();
		emu.regs.sp = 100;
		emu.regs.pc = 0xBEEF;
		let call_a16 = INSTRUCTIONS[0xCD].func.unwrap();
		call_a16(&mut emu, 50);
		assert_eq!(emu.regs.sp, 98);
		assert_eq!(emu.mem.rb(98), 0xEF);
		assert_eq!(emu.mem.rb(99), 0xBE);
		assert_eq!(emu.regs.pc, 50);
	}
	#[test]
	fn test_push_pop() {
		let mut emu = Emulator::new();
		emu.regs.sp = 100;
		*emu.regs.a() = 2;
		let push_af = INSTRUCTIONS[0xF5].func.unwrap();
		push_af(&mut emu, 0);
		unsafe {
			assert_eq!(emu.regs.sp, 98);
			assert_eq!(*emu.regs.af(), 512);
		}
		let pop_hl = INSTRUCTIONS[0xE1].func.unwrap();
		pop_hl(&mut emu, 0);
		unsafe {
			assert_eq!(emu.regs.sp, 100);
			assert_eq!(*emu.regs.hl(), 512);
		}
	}
}