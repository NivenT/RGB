// Not sure where the most appropriate place for this code is, 
// so it is separated it into its own file for now

#[derive(Debug)]
// Is it worth it to just use a bit array?
pub struct ProgramState {
    pub debug:		bool,
    pub done:		bool,
    pub paused:		bool,
    pub adv_frame:	bool,
    pub debug_regs: bool,
    pub speed:		u64,
}

impl ProgramState {
    pub fn new() -> ProgramState {
    	ProgramState {
    		debug: false, 
    		done: false, 
    		paused: false, 
    		adv_frame: false,
            debug_regs: false,
    		speed: 1
    	}
    }
}