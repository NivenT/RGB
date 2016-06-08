use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::debug_output;

pub fn handle_input(events: &mut EventPump) -> bool {
	for event in events.poll_iter() {
        match event {
            Event::Quit{..} => {
                return false;
            },
            Event::KeyDown{keycode: key, ..} => {
            	if let Some(key) = key {
            		if !handle_keydown(key) {
            			return false;
            		}
            	}
            }
            _ => ()
        }
    }
    true
}

fn handle_keydown(key: Keycode) -> bool {
	match key {
		Keycode::D => unsafe {
			debug_output = !debug_output;
			true
		},
		Keycode::Escape => false,
		_ => true
	}
}