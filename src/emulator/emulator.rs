use emulator::registers::Registers;

pub struct Emulator {
	memory:		[u8; 65536],
	controls: 	[u8; 8],
	regs:		Registers
}

#[allow(dead_code)]
impl Emulator {
	pub fn new() -> Emulator {
		Emulator{memory: [0; 65536], controls: [0; 8], regs: Registers::new()}
	}
	pub fn set_controls(&mut self, controls: Vec<u8>) {
		for i in 0..8 {
			self.controls[i] = controls[i];
		}
	}
	pub fn load_game(&mut self, path: String) {

	}
}