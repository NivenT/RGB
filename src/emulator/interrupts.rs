use emulator::memory::Memory;
use emulator::registers::Registers;

pub struct InterruptManager {
	pub ime:	bool
}

impl InterruptManager {
	pub fn new() -> InterruptManager {
		InterruptManager{ime: false}
	}
	pub fn request_interrupt(&self, mem: &mut Memory, id: u8) {
		let interrupt_request_register = mem.rb(0xFF0F);
		mem.wb(0xFF0F, interrupt_request_register | (1 << id));
	}
	pub fn step(&mut self, mem: &mut Memory, regs: &mut Registers) {
		if self.ime {
			for i in 0..5 {
				if (mem.rb(0xFF0F) & (1 << i)) > 0 && (mem.rb(0xFFFF) & (1 << i)) > 0 {
					//Interrupt both requested and enabled
					self.ime = false;
					let request = mem.rb(0xFF0F);
					mem.wb(0xFF0F, request & !(1 << i));

					mem.ww(regs.sp - 2, regs.pc);
					regs.sp -= 2;

					regs.pc = match i {
						0 => 0x40,
						1 => 0x48,
						2 => 0x50,
						4 => 0x60,
						_ => panic!("Unrecognized interrupt: {}", i)
					}
				}
			}
		}
	}
}