use std::io;
use std::io::prelude::*;
use std::str::FromStr;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use programstate::*;
use emulator::Emulator;

fn prompt_for_val<T: FromStr>(prompt: &str) -> Result<T, T::Err> {
    print!("{}", prompt);

    let mut input = String::new();
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut input);

    input.lines().last().unwrap().trim().parse()
}

pub fn handle_input(events: &mut EventPump, state: &mut ProgramState, emu: &mut Emulator) {
	for event in events.poll_iter() {
        match event {
            Event::Quit{..} => {
                state.done = true;
            },
            Event::KeyDown{keycode: key, ..} => {
            	if let Some(key) = key {
                    handle_keydown(key, state, emu);
                    emu.update_keys(key as u8, true);
            	}
            },
            Event::KeyUp{keycode: key, ..} => {
                if let Some(key) = key {
                    emu.update_keys(key as u8, false);
                }
            },
            _ => ()
        }
    }
}

fn handle_keydown(key: Keycode, state: &mut ProgramState, emu: &Emulator) {
	match key {
        Keycode::Num1 => {state.speed = 1},
        Keycode::Num2 => {state.speed = 2},
        Keycode::Num3 => {state.speed = 3},
        Keycode::Num4 => {state.speed = 4},
        Keycode::Num5 => {state.speed = 5},
        Keycode::Num6 => {state.speed = 6},
        Keycode::Num7 => {state.speed = 7},
        Keycode::Num8 => {state.speed = 8},
        Keycode::Num9 => {state.speed = 9},
        Keycode::Num0 => {state.speed = 10},
		Keycode::D => {state.debug = !state.debug},
        Keycode::R => {state.debug_regs = !state.debug_regs},
        Keycode::F => {state.adv_frame = true},
        Keycode::P => {state.paused = !state.paused},
        Keycode::M => {
            //Prompt use for range of memory and then dump memory
            let start: u16 = prompt_for_val("Enter the starting memory address: ").unwrap();
            let stop: u16 = prompt_for_val("Enter the ending memory adress: ").unwrap();
            let diff = stop - start;
            let num_rows = (diff as f64/16f64).ceil() as u16;

            for row in 0..num_rows {
                print!("{:#X}: ", row*16 + start);
                let end = if (diff - row*16) < 16 {diff - row*16} else {16};
                for col in 0..end {
                    print!("{:#X} ", emu.rb(row*16 + col + start));
                }
                println!("");
            }
            println!("");
        },
		Keycode::Escape => {state.done = true},
		_ => ()
	}
}