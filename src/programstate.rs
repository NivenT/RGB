// Not sure where the most appropriate place for this code is, 
// so it is separated it into its own file for now

#[derive(Debug)]
pub struct ProgramState {
    pub debug:		bool,
    pub done:		bool,
    pub paused:		bool,
    pub adv_frame:	bool,
}

impl ProgramState {
    pub fn new() -> ProgramState {
    	ProgramState{debug: false, done: false, paused: false, adv_frame: false}
    }
}