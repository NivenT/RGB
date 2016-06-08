use std::fs::File;
use std::io::prelude::*;

use glium::{Surface, VertexBuffer, IndexBuffer, Program, index};
use glium::texture::*;

use glium_sdl2::SDL2Facade;

use emulator::gpu::Color;

#[derive(Debug, Clone, Copy)]
struct Vertex {
    pos:    [f32; 2],
    uv:     [f32; 2]
}

pub struct Renderer {
	vert_buffer: 	VertexBuffer<Vertex>,
	index_buffer:	IndexBuffer<u8>,
	program:		Program
}

impl Renderer {
	pub fn new(display: &SDL2Facade) -> Renderer {
		implement_vertex!(Vertex, pos, uv);

		let mut vert_shader_src = String::new();
		let mut frag_shader_src = String::new();

		let mut vert_file = File::open("shaders/simp.vert").unwrap();
		let mut frag_file = File::open("shaders/simp.frag").unwrap();
		let _ = vert_file.read_to_string(&mut vert_shader_src);
		let _ = frag_file.read_to_string(&mut frag_shader_src);

		let program = Program::from_source(display, &vert_shader_src, &frag_shader_src, None).unwrap();

		let v1 = Vertex{pos: [-1.0,  1.0], uv: [0.0, 0.0]};
		let v2 = Vertex{pos: [ 1.0,  1.0], uv: [1.0, 0.0]};
		let v3 = Vertex{pos: [ 1.0, -1.0], uv: [1.0, 1.0]};
		let v4 = Vertex{pos: [-1.0, -1.0], uv: [0.0, 1.0]};
		let vertices = vec![v1, v2, v3, v4];

		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
		let index_buffer = IndexBuffer::new(display,
										 	index::PrimitiveType::TrianglesList,
										 	&[0u8,1,2, 2,3,0]).unwrap();

		Renderer{vert_buffer: vertex_buffer, index_buffer: index_buffer, program: program}
	}
	fn make_texture(display: &SDL2Facade, screen: &[[Color; 160]; 144]) -> Texture2d {
	    let raw = screen.into_iter()
	    				.flat_map(|row| {
	    					row.into_iter()
	    					   .map(|&col| {
	    					   		(col as u8, col as u8, col as u8)
	    					   })
	    				})
	    				.collect::<Vec<_>>();
	    let image = RawImage2d::from_raw_rgb(raw, (160, 144));
	    Texture2d::new(display, image).unwrap()
	}

	pub fn render(&self, display: &SDL2Facade, screen: &[[Color; 160]; 144]) {
		let texture = Renderer::make_texture(display, screen);

		let mut target = display.draw();
		target.draw(&self.vert_buffer, &self.index_buffer, &self.program,
						&uniform!{sample: &texture}, &Default::default()).unwrap();
	    target.finish().unwrap();
	}
}

	