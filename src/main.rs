#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate tini;

mod emulator;
mod input;
mod rendering;

use glium_sdl2::DisplayBuild;
use tini::Ini;

use emulator::emulator::Emulator;
use input::*;
use rendering::*;

pub static mut debug_output: bool = false;

fn main() {
	let config = Ini::from_file("settings.ini").unwrap();
	let game_path: String = config.get("game", "game").unwrap();
	let buttons = ["up", "down", "left", "right", "a", "b", "start", "select"];
	let controls: Vec<u8> = buttons.iter()
								   .map(|a| config.get("controls", a).unwrap())
								   .collect();

	let mut emu = Emulator::new();
	emu.set_controls(controls);
    emu.load_game(game_path.clone());

	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display = video_subsystem.window("Rust Gameboy", 800, 600)
                                 .resizable()
                                 .build_glium()
                                 .unwrap();

	let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();
    let renderer = Renderer::new(&display);
    while running {
        running = handle_input(&mut event_pump);
        emu.emulate_cycle();
        renderer.render(&display, emu.gpu.get_screen());
    }
}