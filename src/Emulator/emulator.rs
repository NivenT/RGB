use Emulator::registers::Registers;

pub struct Emulator {
	memory:	[u8; 65536],
	regs:	Registers,
	pc:		u16,
	sp:     u16
}

impl Emulator {

}