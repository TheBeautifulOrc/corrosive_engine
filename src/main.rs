use std::fs::read_to_string;
use std::time::{Duration, Instant};

use glium::{glutin, index, uniforms};
use glium::{implement_vertex, uniform};
use glium::{Display, Program, Surface, VertexBuffer};

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

#[derive(Copy, Clone, Debug)]
struct Vertex {
	position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
	// Setup graphics pipeline
	let mut event_loop = EventLoop::new();
	let window_builer = WindowBuilder::new();
	let context_builder = ContextBuilder::new();
	let display = Display::new(window_builer, context_builder, &event_loop)
		.expect("failed to create display");

	// Define triangle
	let vertex1 = Vertex {
		position: [-0.5, -0.5],
	};
	let vertex2 = Vertex {
		position: [0.0, 0.5],
	};
	let vertex3 = Vertex {
		position: [0.5, -0.25],
	};
	let shape = vec![vertex1, vertex2, vertex3];

	let vertex_buffer =
		VertexBuffer::new(&display, &shape).expect("failed to create vertex buffer");

	let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

	// Import shader source code and compile into shader program
	let vertex_shader =
		read_to_string("./shaders/vertex.glsl").expect("failed to read vertex shader");
	let fragment_shader =
		read_to_string("./shaders/fragment.glsl").expect("failed to read fragment shader");
	let program = Program::from_source(&display, &vertex_shader, &fragment_shader, None)
		.expect("failed to create shader program");

	let mut tri_pos: [f32; 2] = [-0.5, -0.5];
	let mut last_frame = Instant::now();

	// Main loop
	event_loop.run(move |event, _, control_flow| {
		// Handle events
		match event {
			// Window event
			Event::WindowEvent { event, .. } => match event {
				// Should program terminate?
				WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit;
					return;
				}
				_ => return,
			},
			// Other event
			_ => (),
		}

		// Timer for next frame (@60 Hz)
		let now = Instant::now();
		println!("{:?}", now - last_frame);
		last_frame = now;
		let next_frame_time = now + Duration::from_nanos(16_666_667);
		// Sleep until next frame is due
		*control_flow = ControlFlow::WaitUntil(next_frame_time);

		for i in 0..2 {
			tri_pos[i] += 0.002;
			if tri_pos[i] >= 0.5 {
				tri_pos[i] = -0.5;
			}
		}

		// Drawing operation
		// Clear canvas
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 0.0, 1.0);
		// Draw triangle
		target
			.draw(
				&vertex_buffer,
				&indices,
				&program,
				&uniform! { tri_pos: tri_pos },
				&Default::default(),
			)
			.expect("failed to draw triangle");
		// Display staged drawing
		target.finish().unwrap();
	});
}
