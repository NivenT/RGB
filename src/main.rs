#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate glium_sdl2;
extern crate sdl2;
extern crate tini;
extern crate time;
extern crate fps_clock;

mod emulator;
mod input;
mod rendering;
mod programstate;
mod utils;

use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use glium_sdl2::DisplayBuild;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use tini::Ini;
use time::PreciseTime;

use emulator::{Emulator, SoundManager};
use input::*;
use rendering::*;
use programstate::*;

const FPS: u32 = 60;
// A real Gameboy executes this many cycles a second
const CYCLES_PER_SECOND: u64 = 4194304;
const CYCLES_PER_FRAME: u64 = CYCLES_PER_SECOND/FPS as u64;

fn main() {
    let mut state = ProgramState::new();
    let mut dstate = DebugState::new();

	let config = Ini::from_file("settings.ini").unwrap();
	let game_path: String = config.get("system", "game").unwrap();
    let bios_path: String = config.get("system", "bios").unwrap_or("".to_string());
	let buttons = ["right", "left", "up", "down", "a", "b", "select", "start"];
	let controls: Vec<u8> = buttons.iter()
								   .map(|a| config.get("controls", a).unwrap())
								   .collect();
    let white: String = config.get("screen", "white").unwrap_or("4D8210".to_string());
    let black: String = config.get("screen", "black").unwrap_or("1F3C1F".to_string());

    let white = u32::from_str_radix(&white, 16).unwrap();
    let black = u32::from_str_radix(&black, 16).unwrap();

    let bios_breakpoint = config.get::<String>("debug", "bios_breakpoint").map_or(false, |s| {
    	s.to_lowercase() == "true"
    });
    let unimpl_instr_breakpoint = config.get::<String>("debug", "unimplemented_instruction_breakpoint").map_or(false, |s| {
    	s.to_lowercase() == "true"
    });
    let inf_loop_breakpoint = config.get::<String>("debug", "infinite_loop_breakpoint").map_or(false, |s| {
    	s.to_lowercase() == "true"
    });
    let dev_keys_enabled = config.get::<String>("debug", "enable_development_keys").map_or(false, |s| {
    	s.to_lowercase() == "true"
    });
    let only_gb_buttons = config.get::<String>("debug", "only_gameboy_buttons").map_or(false, |s| {
    	s.to_lowercase() == "true"
    });

    if let Ok(mut file) = File::create("disassembly.txt") {
        let _ = file.write(Emulator::disassemble_file(&game_path.clone()).as_ref());
    }

	let mut emu = Emulator::new(bios_breakpoint, unimpl_instr_breakpoint, inf_loop_breakpoint);
	emu.set_controls(controls);
    emu.load_bios(bios_path);
    emu.load_game(game_path);
    let emu = Arc::new(Mutex::new(emu));

	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None
    };
    let device = audio_subsystem.open_playback(None, &desired_spec, |_spec| {
            SoundManager::new(Arc::clone(&emu))
    }).unwrap();

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

    device.resume();
    println!("Using OpenGL Version: {}", display.get_opengl_version_string());
    let mut fps = fps_clock::FpsClock::new(FPS);
    while !state.done {
        let mut emu = emu.lock().unwrap();

        if start.to(PreciseTime::now()).num_seconds() >= 1 {
            let acc = 100f64*(cycles_per_second as f64/(CYCLES_PER_SECOND*emu.get_speed()) as f64);
            let title = if state.paused {"Paused"} else {"Rust Gameboy"};
            let _ = display.window_mut().set_title(&format!("{} ({:.2}%)", title, acc));

            cycles_per_second = 0;
            start = PreciseTime::now();

            emu.save_game();
        }
        handle_input(&mut event_pump, &mut state, &mut dstate, &mut emu, dev_keys_enabled, only_gb_buttons);
        
        while (!state.paused || state.adv_frame) && cycles_this_frame < CYCLES_PER_FRAME*emu.get_speed() {
            cycles_this_frame += emu.step(&mut state, &mut dstate);
            state.adv_frame = false;
        }
        if frames_until_render == 0 {
            renderer.render(&display, emu.get_screen(), &state, &mut dstate);
            fps.tick();
        }

        frames_until_render = (frames_until_render+1)%state.speed;
        cycles_per_second += cycles_this_frame;
        cycles_this_frame = 0;
    }
}
