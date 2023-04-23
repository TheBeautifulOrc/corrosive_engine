use nalgebra::Vector2;
use winit::event::{Event, WindowEvent};

mod editor;
mod engine;

use engine::core::{renderer::Renderer, window};

// Test
use bytemuck::{Pod, Zeroable};
#[repr(C)]
#[derive(Default, Clone, Copy, Zeroable, Pod)] // Vertex
struct Vertex {
	position: [f32; 3],
}


fn main() {
	// Create editor main window
	let (window, event_loop) =
		window::create(&Vector2::new(800, 600), "Corrosive Editor".to_string());

	// Create Vulkan renderer
	let renderer = Renderer::default();

	// Test: Create vertices
	let v1 = Vertex { position: [-1.0, -1.0, 0.0] };
	let v2 = Vertex { position: [1.0, 1.0, 0.0] };
	let v3 = Vertex { position: [1.0, -1.0, 0.0] };

	use vulkano::buffer::{Buffer, BufferUsage};
	// let vertex_buffer = Buffer::from_iter(

	// )

	// Main execution loop
	event_loop.run(move |event, _, control_flow| {
		// Wait for events to happen to save on performance
		control_flow.set_wait();

		match event {
			// Close application
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => {
				control_flow.set_exit();
			}
			// Update application and request redraw
			Event::MainEventsCleared => {
				window.request_redraw();
			}
			// Redraw window
			Event::RedrawRequested(_) => {}
			// Default
			_ => (),
		}
	});
}
