use std::fs::File;
use std::io::prelude::*;

use glium::{Surface, VertexBuffer, IndexBuffer, Program, index, Frame};
use glium::texture::*;

use glium_text;
use glium_text::{TextSystem, FontTexture, TextDisplay};

use glium_sdl2::SDL2Facade;

use emulator::Color;

use super::{ProgramState, DebugState};
use super::utils::*;

#[derive(Debug, Clone, Copy)]
struct Vertex {
    pos:    [f32; 2],
    uv:     [f32; 2]
}

implement_vertex!(Vertex, pos, uv);

pub struct Renderer {
	vert_buffer: VertexBuffer<Vertex>,
	half_buffer: VertexBuffer<Vertex>,
	index_buffer: IndexBuffer<u8>,
	program: Program,
	system: TextSystem,
	font: FontTexture,
	white: (f32, f32, f32),
	black: (f32, f32, f32)
}

impl Renderer {
	pub fn new(display: &SDL2Facade, white: u32, black: u32) -> Renderer {
		let mut vert_shader_src = String::new();
		let mut frag_shader_src = String::new();

		let mut vert_file = File::open("shaders/simp.vert").unwrap();
		let mut frag_file = File::open("shaders/simp.frag").unwrap();
		let _ = vert_file.read_to_string(&mut vert_shader_src);
		let _ = frag_file.read_to_string(&mut frag_shader_src);

		let program = Program::from_source(display, &vert_shader_src, &frag_shader_src, None).unwrap();

		// normally fill screen with emulator
		let v1 = Vertex{pos: [-1.0,  1.0], uv: [0.0, 0.0]};
		let v2 = Vertex{pos: [ 1.0,  1.0], uv: [1.0, 0.0]};
		let v3 = Vertex{pos: [ 1.0, -1.0], uv: [1.0, 1.0]};
		let v4 = Vertex{pos: [-1.0, -1.0], uv: [0.0, 1.0]};
		let vertices = vec![v1, v2, v3, v4];
		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

		// in debug, only use left part of screen
		let end = 2.0 * (1.0 - PORTION_DEBUG) - 1.0;
		let v1 = Vertex{pos: [-1.0,  1.0], uv: [0.0, 0.0]};
		let v2 = Vertex{pos: [ end,  1.0], uv: [1.0, 0.0]};
		let v3 = Vertex{pos: [ end, -1.0], uv: [1.0, 1.0]};
		let v4 = Vertex{pos: [-1.0, -1.0], uv: [0.0, 1.0]};
		let vertices = vec![v1, v2, v3, v4];
		let half_buffer = VertexBuffer::new(display, &vertices).unwrap();

		let index_buffer = IndexBuffer::new(display,
										 	index::PrimitiveType::TrianglesList,
										 	&[0u8,1,2, 2,3,0]).unwrap();
		// Extract RGB information from hex colors
		let white = (((white >> 16) & 0xFF) as f32, ((white >> 8) & 0xFF) as f32, (white & 0xFF) as f32);
		let black = (((black >> 16) & 0xFF) as f32, ((black >> 8) & 0xFF) as f32, (black & 0xFF) as f32);
		println!("Using {:?} and {:?} as white and black, respectively.", white, black);

		let system = TextSystem::new(display);

		let font_file = File::open("fonts/font.otf").unwrap();
		let font = FontTexture::new(display, font_file, FONT_SIZE).unwrap();

		Renderer {
			vert_buffer: vertex_buffer,
			half_buffer: half_buffer, 
			index_buffer: index_buffer, 
			program: program,
			system: system,
			font: font,
			white: white, 
			black: black
		}
	}
	fn make_texture(&self, display: &SDL2Facade, screen: &[[Color; 160]; 144]) -> Texture2d {
	    let raw = screen.into_iter()
	    				.flat_map(|row| {
	    					row.into_iter()
	    					   .map(|&col| {
	    					   		match col {
	    					   			Color::CGB(red, green ,blue) => (red, green, blue),
	    					   			_ => {
	    					   				let mix = col.to_f32().unwrap()/255f32;
			    					   		let (white, black) = (self.white, self.black);
			    					   		((white.0*mix + black.0*(1f32-mix)) as u8,
			    					   		 (white.1*mix + black.1*(1f32-mix)) as u8,
			    					   		 (white.2*mix + black.2*(1f32-mix)) as u8)
	    					   			}
	    					   		}
	    					   })
	    				})
	    				.collect::<Vec<_>>();
	    let image = RawImage2d::from_raw_rgb(raw, (160, 144));
	    Texture2d::new(display, image).unwrap()
	}
	fn render_line_of_text(&self, y: f32, text: &str, target: &mut Frame) {
		let length = text.len() as f32;
		let text = TextDisplay::new(&self.system, &self.font, text);

		let start = 1.0 - 2.0 * PORTION_DEBUG;
		// column major (each row is actually a column)
		let transformation = [
			[CHAR_WIDTH*length/text.get_width(), 0.0,           0.0, 0.0],
			[0.0,                                TEXT_HEIGHT,   0.0, 0.0],
			[0.0,                                0.0,           1.0, 0.0],
			[start,                              y-TEXT_HEIGHT, 0.0, 1.0]
		];

		// TODO: Make text color customizable
		let text_color = (1.0, 1.0, 1.0, 1.0);
		glium_text::draw(&text, &self.system, target, transformation, text_color);
	}
	fn display_gameboy(&self, display: &SDL2Facade, target: &mut Frame, screen: &[[Color; 160]; 144], state: &ProgramState) {
		let texture = self.make_texture(display, screen);
		let buf = if state.debug {&self.half_buffer} else {&self.vert_buffer};
		target.draw(buf, &self.index_buffer, &self.program, &uniform!{sample: &texture}, 
					&Default::default()).unwrap();
	}
	fn display_debug_info(&self, target: &mut Frame, dstate: &DebugState) {
		let cursor = if dstate.num_lines - dstate.cursor < NUM_LINES_ON_SCREEN {
			// usizes are unsigned so this subtraction is just wrong (same in input.rs). Oh well...
			max(0, dstate.num_lines - NUM_LINES_ON_SCREEN)
		} else {
			dstate.cursor
		};

		for (i, line) in dstate.buffer.lines().skip(cursor).take(NUM_LINES_ON_SCREEN).enumerate() {
			self.render_line_of_text(1.0 - (i as f32)*LINE_HEIGHT, line, target);
		}
	}

	pub fn render(&self, display: &SDL2Facade, screen: &[[Color; 160]; 144], state: &ProgramState, dstate: &DebugState) {
		let mut target = display.draw();
		target.clear(None, Some((0.0, 0.0, 0.0, 1.0)), false, None, None);

		if state.debug {
			self.display_debug_info(&mut target, dstate);
		}
		self.display_gameboy(display, &mut target, screen, state);

	    target.finish().unwrap();
	}
}

	