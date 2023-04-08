use nalgebra::Vector2;
use winit::event::{Event, WindowEvent};

mod editor;
mod engine;

use engine::core::{renderer::Renderer, window};

fn main() {
	// Create editor main window
	let (window, event_loop) =
		window::create(&Vector2::new(800, 600), "Corrosive Editor".to_string());

	// Create Vulkan renderer
	let renderer = Renderer::default();

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
