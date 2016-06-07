#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate tini;

mod emulator;

use glium::Surface;
use glium_sdl2::DisplayBuild;
use sdl2::event::Event;
use tini::Ini;

use emulator::emulator::Emulator;

pub static mut debug_output: bool = true;

fn main() {
	let config = Ini::from_file("settings.ini").unwrap();
	let game_path: String = config.get("game", "game").unwrap();
	let buttons = ["up", "down", "left", "right", "a", "b", "start", "select"];
	let controls: Vec<u8> = buttons.iter()
								   .map(|a| config.get("controls", a).unwrap())
								   .collect();
	/*
	println!("game_path: {}", game_path);
	for (key, val) in buttons.iter().zip(controls.iter()) {
		println!("{}: {}", key, val);
	}
	*/

	let mut emu = Emulator::new();
	emu.set_controls(controls);
    emu.load_game(game_path.clone());

	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display = video_subsystem.window("My window", 800, 600)
        .resizable()
        .build_glium()
        .unwrap();

	let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();
    while running {
		for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => {
                    running = false;
                },
                Event::KeyDown{keycode: key, ..} => {
                	if let Some(key) = key {
                		//println!("key pressed: {:?} ({})", key, key as u8);

                        if key as u8 == 100 { //D
                            unsafe{debug_output = !debug_output;}
                        }
                	}
                }
                _ => ()
            }
        }

        let mut target = display.draw();
        target.clear_color(0.1, 0.1, 0.1, 1.0f32);
        target.finish().unwrap();

        emu.emulate_cycle();
    }
}