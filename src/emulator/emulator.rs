use emulator::registers::Registers;

pub struct Emulator {
	memory:	[u8; 65536],
	regs:	Registers,
	pc:		u16,
	sp:     u16
}

impl Emulator {
	pub fn new() -> Emulator {
		Emulator{memory: [0; 65536], regs: Registers::new(), pc: 0, sp: 0}
	}
}