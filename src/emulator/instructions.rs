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
    ($regs:expr, $reg:ident) => {
    	*$regs.a() ^= *$regs.$reg();

    	$regs.clear_flags(ALL_FLAGS);
    	if *$regs.a() == 0 {$regs.set_flags(ZERO_FLAG);}
    }
}

pub type InstructionFunc = Option<&'static Fn(&mut Emulator, u16) -> ()>;

#[derive(Copy, Clone)]
pub struct Instruction {
	pub name:			&'static str,
	pub operand_length:	u8,
	pub func:			InstructionFunc
}

#[allow(dead_code)]
pub const INSTRUCTIONS: [Instruction; 256] = [
	//0x00
	new_instruction!("NOP", 0, Some(&|_,_| ())),	
	new_instruction!("LD BC,d16, 0x%04X", 2, None),
	new_instruction!("LD (BC), A", 0, None),
	new_instruction!("INC BC", 0, None),
	new_instruction!("INC B", 0, None),
	new_instruction!("DEC B", 0, None),
	new_instruction!("LD B,d8", 1, None),
	new_instruction!("RLCA", 0, None),
	//0x08
	new_instruction!("LD (a16),SP", 2, None),
	new_instruction!("ADD HL,BC", 0, None),
	new_instruction!("LD A,(BC)", 0, None),
	new_instruction!("DEC BC", 0, None),
	new_instruction!("INC C", 0, None),
	new_instruction!("DEC C", 0, None),
	new_instruction!("LD C,d8", 1, None),
	new_instruction!("RRCA", 0, None),
	//0x10
	new_instruction!("STOP 0", 1, None),			
	new_instruction!("LD DE,d16", 2, None),
	new_instruction!("LD (DE),A", 0, None),
	new_instruction!("INC DE", 0, None),
	new_instruction!("INC D", 0, None),
	new_instruction!("DEC D", 0, None),
	new_instruction!("LD D,d8", 1, None),
	new_instruction!("RLA", 0, None),
	//0x18
	new_instruction!("JR r8", 1, None),				
	new_instruction!("ADD HL,DE", 0, None),
	new_instruction!("LD A,(DE)", 0, None),
	new_instruction!("DEC DE", 0, None),
	new_instruction!("INC E", 0, None),
	new_instruction!("DEC E", 0, None),
	new_instruction!("LD E,d8", 1, None),
	new_instruction!("RRA", 0, None),
	//0x20
	new_instruction!("JR NZ,r8", 1, None),			
	new_instruction!("LD HL,d16", 2, Some(&ld_hl)),
	new_instruction!("LD (HL+),A", 0, None),
	new_instruction!("INC HL", 0, None),
	new_instruction!("INC H", 0, None),
	new_instruction!("DEC H", 0, None),
	new_instruction!("LD H,d8", 1, None),
	new_instruction!("DAA", 0, None),
	//0x28
	new_instruction!("JR Z,r8", 1, None),			
	new_instruction!("ADD HL,HL", 0, None),
	new_instruction!("LD A,(HL+)", 0, None),
	new_instruction!("DEC HL", 0, None),
	new_instruction!("INC L", 0, None),
	new_instruction!("DEC L", 0, None),
	new_instruction!("LD L,d8", 1, None),
	new_instruction!("CPL", 0, None),
	//0x30
	new_instruction!("JP NC,r8", 1, None),			
	new_instruction!("LD SP,d16", 2, Some(&ld_sp)),
	new_instruction!("LD (HL-),A", 0, Some(&ld_hld_a)),
	new_instruction!("INC SP", 0, None),
	new_instruction!("INC (HL)", 0, None),
	new_instruction!("DEC (HL)", 0, None),
	new_instruction!("LD (HL),d8", 1, None),
	new_instruction!("SCF", 0, None),
	//0x38
	new_instruction!("JR C,r8", 1, None),			
	new_instruction!("ADD HL,SP", 0, None),
	new_instruction!("LD A,(HL-)", 0, None),
	new_instruction!("DEC SP", 0, None),
	new_instruction!("INC A", 0, None),
	new_instruction!("DEC A", 0, None),
	new_instruction!("LD A,d8", 1, None),
	new_instruction!("CCF", 0, None),
	//0x40
	new_instruction!("LD B,B", 0, None),			
	new_instruction!("LD B,C", 0, None),
	new_instruction!("LD B,D", 0, None),
	new_instruction!("LD B,E", 0, None),
	new_instruction!("LD B,H", 0, None),
	new_instruction!("LD B,L", 0, None),
	new_instruction!("LD B,(HL)", 0, None),
	new_instruction!("LD B,A", 0, None),
	//0x48
	new_instruction!("LD C,B", 0, None),			
	new_instruction!("LD C,C", 0, None),
	new_instruction!("LD C,D", 0, None),
	new_instruction!("LD C,E", 0, None),
	new_instruction!("LD C,H", 0, None),
	new_instruction!("LD C,L", 0, None),
	new_instruction!("LD C,(HL)", 0, None),
	new_instruction!("LD C,A", 0, None),
	//0x50
	new_instruction!("LD D,B", 0, None),			
	new_instruction!("LD D,C", 0, None),
	new_instruction!("LD D,D", 0, None),
	new_instruction!("LD D,E", 0, None),
	new_instruction!("LD D,H", 0, None),
	new_instruction!("LD D,L", 0, None),
	new_instruction!("LD D,(HL)", 0, None),
	new_instruction!("LD D,A", 0, None),
	//0x58
	new_instruction!("LD E,B", 0, None),			
	new_instruction!("LD E,C", 0, None),
	new_instruction!("LD E,D", 0, None),
	new_instruction!("LD E,E", 0, None),
	new_instruction!("LD E,H", 0, None),
	new_instruction!("LD E,L", 0, None),
	new_instruction!("LD E,(HL)", 0, None),
	new_instruction!("LD E,A", 0, None),
	//0x60
	new_instruction!("LD H,B", 0, None),			
	new_instruction!("LD H,C", 0, None),
	new_instruction!("LD H,D", 0, None),
	new_instruction!("LD H,E", 0, None),
	new_instruction!("LD H,H", 0, None),
	new_instruction!("LD H,L", 0, None),
	new_instruction!("LD H,(HL)", 0, None),
	new_instruction!("LD H,A", 0, None),
	//0x68
	new_instruction!("LD L,B", 0, None),			
	new_instruction!("LD L,C", 0, None),
	new_instruction!("LD L,D", 0, None),
	new_instruction!("LD L,E", 0, None),
	new_instruction!("LD L,H", 0, None),
	new_instruction!("LD L,L", 0, None),
	new_instruction!("LD L,(HL)", 0, None),
	new_instruction!("LD L,A", 0, None),
	//0x70
	new_instruction!("LD (HL),B", 0, None),			
	new_instruction!("LD (HL),C", 0, None),
	new_instruction!("LD (HL),D", 0, None),
	new_instruction!("LD (HL),E", 0, None),
	new_instruction!("LD (HL),H", 0, None),
	new_instruction!("LD (HL),L", 0, None),
	new_instruction!("HALT", 0, None),
	new_instruction!("LD (HL),A", 0, None),
	//0x78
	new_instruction!("LD A,B", 0, None),			
	new_instruction!("LD A,C", 0, None),
	new_instruction!("LD A,D", 0, None),
	new_instruction!("LD A,E", 0, None),
	new_instruction!("LD A,H", 0, None),
	new_instruction!("LD A,L", 0, None),
	new_instruction!("LD A,(HL)", 0, None),
	new_instruction!("LD A,A", 0, None),
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
	new_instruction!("XOR B", 0, None),				
	new_instruction!("XOR C", 0, None),
	new_instruction!("XOR D", 0, None),
	new_instruction!("XOR E", 0, None),
	new_instruction!("XOR H", 0, None),
	new_instruction!("XOR L", 0, None),
	new_instruction!("XOR (HL)", 0, None),
	new_instruction!("XOR A", 0, Some(&xor_a)),
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
	new_instruction!("CP B", 0, None),				
	new_instruction!("CP C", 0, None),
	new_instruction!("CP D", 0, None),
	new_instruction!("CP E", 0, None),
	new_instruction!("CP H", 0, None),
	new_instruction!("CP L", 0, None),
	new_instruction!("CP (HL)", 0, None),
	new_instruction!("CP A", 0, None),
	//0xC0
	new_instruction!("RET NZ", 0, None),			
	new_instruction!("POP BC", 0, None),
	new_instruction!("JP NZ,a16", 2, None),
	new_instruction!("JP a16", 2, None),
	new_instruction!("CALL NZ,a16", 2, None),
	new_instruction!("PUSH BC", 0, None),
	new_instruction!("ADD A,d8", 1, None),
	new_instruction!("RST 00H", 0, None),
	//0xC8
	new_instruction!("RET Z", 0, None),				
	new_instruction!("RET", 0, None),
	new_instruction!("JP Z,a16", 2, None),
	new_instruction!("PREFIX CB", 1, Some(&cb)),
	new_instruction!("CALL Z,a16", 2, None),
	new_instruction!("CALL a16", 2, None),
	new_instruction!("ADC A,d8", 1, None),
	new_instruction!("RST 00H", 0, None),
	//0xD0
	new_instruction!("RET NC", 0, None),			
	new_instruction!("POP DE", 0, None),
	new_instruction!("JP NC,a16", 2, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("CALL NC,a16", 2, None),
	new_instruction!("PUSH DE", 0, None),
	new_instruction!("SUB d8", 1, None),
	new_instruction!("RST 10H", 0, None),
	//0xD8
	new_instruction!("RET C", 0, None),				
	new_instruction!("RETI", 0, None),
	new_instruction!("JP C,a16", 2, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("CALL C,a16", 2, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("SBC A,d8", 1, None),
	new_instruction!("RST 10H", 0, None),
	//0xE0
	new_instruction!("LDH (a8),A", 1, None),		
	new_instruction!("POP HL", 0, None),
	new_instruction!("LD (C),A", 1, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("PUSH HL", 0, None),
	new_instruction!("AND d8", 1, None),
	new_instruction!("RST 20H", 0, None),
	//0xE8
	new_instruction!("ADD SP,r8", 1, None),			
	new_instruction!("JP (HL)", 0, None),
	new_instruction!("LD (a16),A", 2, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("XOR d8", 1, None),
	new_instruction!("RST 20H", 0, None),
	//0xF0
	new_instruction!("LDH A,(a8)", 1, None),		
	new_instruction!("POP AF", 0, None),
	new_instruction!("LD A,(C)", 1, None),
	new_instruction!("DI", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("PUSH AF", 0, None),
	new_instruction!("OR d8", 1, None),
	new_instruction!("RST 30H", 0, None),
	//0xF8
	new_instruction!("LD HL,SP+r8", 1, None),	
	new_instruction!("LD SP,HL", 0, None),
	new_instruction!("LD A,(a16)", 2, None),
	new_instruction!("EI", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("NO_INSTRUCTION", 0, None),
	new_instruction!("CP d8", 1, None),
	new_instruction!("RST 30H", 0, None),
];

//0x21
fn ld_hl(emu: &mut Emulator, operand: u16) {
	unsafe{*emu.regs.hl() = operand;}
}

//0x31
fn ld_sp(emu: &mut Emulator, operand: u16) {
	emu.regs.sp = operand;
}

//0x32
fn ld_hld_a(emu: &mut Emulator, _: u16) {
	unsafe{
		emu.memory[*emu.regs.hl() as usize] = *emu.regs.a();
		*emu.regs.hl() -= 1;
	}
}

//0xAF
fn xor_a(emu: &mut Emulator, _: u16) {
	xor!(emu.regs, a);
}

//0xCB
fn cb(emu: &mut Emulator, operand: u16) {
	let instruction = CB_INSTRUCTIONS[operand as usize];
	if let Some(func) = instruction.func {
		func(emu);
	} else {
		println!("Unimplemented function at memory address {:#X} [{:#X} {:#X} ({})]", 
			emu.regs.pc-1, 0xCB, operand, instruction.name);
		panic!("");
	}
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
		*emu.regs.l() = 5;
		*emu.regs.a() = 18;
		unsafe{
			assert_eq!(*emu.regs.hl(), 5);
			assert_eq!(emu.memory[5], BIOS[5]);
			let ld_hld_a = INSTRUCTIONS[0x32].func.unwrap();
			ld_hld_a(&mut emu, 0);
			assert_eq!(*emu.regs.hl(), 4);
			assert_eq!(emu.memory[5], 18);
		}

	}
}