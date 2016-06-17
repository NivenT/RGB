use std::io;
use std::io::prelude::*;
use std::str::FromStr;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use programstate::*;
use emulator::memory::Memory;

fn prompt_for_val<T>(prompt: &str) -> Result<T, T::Err>
    where T: FromStr {
    print!("{}", prompt);

    let mut input = String::new();
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut input);

    input.lines().last().unwrap().trim().parse()
}

pub fn handle_input(events: &mut EventPump, state: &mut ProgramState, mem: &Memory) {
	for event in events.poll_iter() {
        match event {
            Event::Quit{..} => {
                state.done = true;
            },
            Event::KeyDown{keycode: key, ..} => {
            	if let Some(key) = key {
                    handle_keydown(key, state, mem);
            	}
            }
            _ => ()
        }
    }
}

fn handle_keydown(key: Keycode, state: &mut ProgramState, mem: &Memory) {
	match key {
		Keycode::D => {state.debug = !state.debug},
        Keycode::P => {state.paused = !state.paused},
        Keycode::F => {state.adv_frame = true},
        Keycode::M => {
            //Prompt use for range of memory and then dump memory
            let start: u16 = prompt_for_val("Enter the starting memory address: ").unwrap();
            let stop: u16 = prompt_for_val("Enter the ending memory adress: ").unwrap();
            let diff = stop - start;
            let num_rows = (diff as f64/16f64).ceil() as u16;

            for row in 0..num_rows {
                print!("{:#X}: ", row*16 + start);
                let min = if (diff - row*16) < 16 {diff - row*16} else {16};
                for col in 0..min {
                    print!("{:#X} ", mem.rb(row*16 + col + start));
                }
                println!("");
            }
        },
		Keycode::Escape => {state.done = true},
		_ => ()
	}
}