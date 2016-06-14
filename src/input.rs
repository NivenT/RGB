use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use programstate::*;

pub fn handle_input(events: &mut EventPump, state: &mut ProgramState) {
	for event in events.poll_iter() {
        match event {
            Event::Quit{..} => {
                state.done = true;
            },
            Event::KeyDown{keycode: key, ..} => {
            	if let Some(key) = key {
                    handle_keydown(key, state);
            	}
            }
            _ => ()
        }
    }
}

fn handle_keydown(key: Keycode, state: &mut ProgramState) {
	match key {
		Keycode::D => {state.debug = !state.debug},
        Keycode::P => {state.paused = !state.paused},
        Keycode::F => {state.adv_frame = true},
        Keycode::M => {
            //Prompt use for range of memory and then dump memory
        },
		Keycode::Escape => {state.done = true},
		_ => ()
	}
}