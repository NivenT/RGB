use emulator::memory::Memory;
use emulator::interrupts::InterruptManager;

pub struct Timers {
	div_counter:	i16, //Update every 256 cycles (4194304/16384)
	tima_counter:	i16
}

impl Timers {
	pub fn new() -> Timers {
	    Timers{div_counter: 0, tima_counter: 0}
	}
	pub fn step(&mut self, mem: &mut Memory, im: &InterruptManager, cycles: i16) {
		self.div_counter -= cycles;
		if self.div_counter <= 0 {
			self.div_counter = 256;
			let div_reg = mem.rb(0xFF04);
			mem.wb(0xFF04, div_reg.wrapping_add(1));
		}

		let tac = mem.rb(0xFF07);
		if tac & 4 > 0 {
			self.tima_counter -= cycles;
		}
		if self.tima_counter <= 0 {
			self.tima_counter = match tac & 0x3 {
				0 => (41494304/4096) as i16,
				1 => (41494304/262144) as i16,
				2 => (41494304/65536) as i16,
				3 => (41494304/16384) as i16,
				_ => panic!("Invalid lower 2 bits for TAC")
			};
			let tima = mem.rb(0xFF05);
			if tima == 255 {
				let tma = mem.rb(0xFF06);
				mem.wb(0xFF05, tma);
				im.request_interrupt(mem, 2);
			} else {
				mem.wb(0xFF05, tima+1);
			}
		}
	}
}