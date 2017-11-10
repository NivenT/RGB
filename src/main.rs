#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate glium_sdl2;
extern crate sdl2;
extern crate tini;
extern crate time;

mod emulator;
mod input;
mod rendering;
mod programstate;
mod utils;

use std::fs::File;
use std::io::prelude::*;

use glium_sdl2::DisplayBuild;
use tini::Ini;
use time::PreciseTime;

use emulator::Emulator;
use input::*;
use rendering::*;
use programstate::*;

// A real Gameboy executes this many cycles a second
const CYCLES_PER_SECOND: u64 = 4194304;

// SDL Automatically caps FPS to the refresh rate of the screen
// This many cyles should be emulated each frame to keep the emulator
// consistent with a real gameboy (assuming 60 FPS)
const CYCLES_PER_FRAME: u64 = 69905;

fn main() {
    let mut state = ProgramState::new();
    let mut dstate = DebugState::new();

	let config = Ini::from_file("settings.ini").unwrap();
	let game_path: String = config.get("system", "game").unwrap();
    let bios_path: String = config.get("system", "bios").unwrap();
	let buttons = ["right", "left", "up", "down", "a", "b", "select", "start"];
	let controls: Vec<u8> = buttons.iter()
								   .map(|a| config.get("controls", a).unwrap())
								   .collect();
    let white: String = config.get("screen", "white").unwrap();
    let black: String = config.get("screen", "black").unwrap();

    let white = u32::from_str_radix(&white, 16).unwrap();
    let black = u32::from_str_radix(&black, 16).unwrap();

    let bios_breakpoint = config.get::<String>("debug", "bios_breakpoint").map_or(false, |s| {
    	s.to_lowercase() == "true"
    });

    if let Ok(mut file) = File::create("disassembly.txt") {
        let _ = file.write(Emulator::disassemble_file(&game_path.clone()).as_ref());
    }

	let mut emu = Emulator::new(bios_breakpoint);
	emu.set_controls(controls);
    emu.load_bios(bios_path);
    emu.load_game(game_path);

	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut display = video_subsystem.window("Rust Gameboy", 800, 600)
                                     .resizable()
                                     .build_glium()
                                     .unwrap();

    let mut start = PreciseTime::now();

    let mut cycles_this_frame = 0;
    let mut cycles_per_second = 0;
    let mut frames_until_render = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    let renderer = Renderer::new(&display, white, black);
    while !state.done {
        if start.to(PreciseTime::now()).num_seconds() >= 1 {
            let acc = 100f64*(cycles_per_second as f64/CYCLES_PER_SECOND as f64);
            let title = if state.paused {"Paused"} else {"Rust Gameboy"};
            let _ = display.window_mut().set_title(&format!("{} ({:.2}%)", title, acc));

            cycles_per_second = 0;
            start = PreciseTime::now();

            emu.save_game();
        }
        handle_input(&mut event_pump, &mut state, &mut dstate, &mut emu);
        
        while (!state.paused || state.adv_frame) && cycles_this_frame < CYCLES_PER_FRAME {
            cycles_this_frame += emu.step(&mut state, &mut dstate);
            state.adv_frame = false;
        }
        if frames_until_render == 0 {
            renderer.render(&display, emu.get_screen(), &state, &mut dstate);
        }

        frames_until_render = (frames_until_render+1)%state.speed;
        cycles_per_second += cycles_this_frame;
        cycles_this_frame = 0;
    }
}
