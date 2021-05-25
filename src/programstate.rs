// TODO: Get rid of this file and implement all this functionality in a nicer way
// Alternative TODO: Never clean this up, but make sure to do things better next time you write an emulator

// Not sure where the most appropriate place for this code is,
// so it is separated it into its own file for now

use utils::*;

#[derive(Debug)]
// Is it worth it to just use a bit array?
pub struct ProgramState {
    pub debug: bool,
    pub done: bool,
    pub paused: bool,
    pub adv_frame: bool,
    pub debug_regs: bool,
    pub speed: u64,
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
            speed: 1,
        }
    }
}

#[derive(Debug)]
pub struct DebugState {
    pub buffer: String,
    pub cursor: usize,
    pub num_lines: usize,
}

impl DebugState {
    pub fn new() -> DebugState {
        DebugState {
            buffer: String::new(),
            cursor: 0,
            num_lines: 0,
        }
    }
    // TODO: Make faster. Maybe replace buffer with array and keep track of "top" of buffer
    pub fn add_text(&mut self, text: &str, num_lines: usize) {
        if self.num_lines + num_lines > MAX_DEBUG_BUFFER_SIZE {
            let extra = self.num_lines + num_lines - MAX_DEBUG_BUFFER_SIZE;
            let extra_lines = self.buffer.match_indices('\n').nth(extra).unwrap().0;

            self.buffer = self.buffer.split_off(extra_lines);
            self.num_lines -= extra;
            self.cursor = max(0, self.cursor - extra);
        }

        self.buffer += text;
        self.cursor += if self.cursor == self.num_lines {
            num_lines
        } else {
            0
        };
        self.num_lines += num_lines;
    }
}
