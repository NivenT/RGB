// TODO: Get rid of this file and implement all this functionality in a nicer way
// Alternative TODO: Never clean this up, but make sure to do things better next time you write an emulator

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
    // TODO: Replace parameterless new() with Default trait
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

#[derive(Debug)]
pub struct DebugState {
    // I should probably cap the length (i.e. number of lines) of this thing
    pub buffer: String,
    pub cursor: usize,
    pub num_lines: usize,
    pub at_end: bool,
}

impl DebugState {
    pub fn new() -> DebugState {
        DebugState {
            buffer: String::new(),
            cursor: 0,
            num_lines: 0,
            at_end: true,
        }
    }
}