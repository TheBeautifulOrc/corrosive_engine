use std::error::Error;
use std::sync::Arc;
use std::cell::RefCell;

use nalgebra::{Vector2};

use winit::{
	window::Window,
	event::{Event, WindowEvent},
	event_loop::{EventLoop, EventLoopWindowTarget, ControlFlow},
	error::OsError
};

use super::{
	renderer::Renderer,
	window
};

pub struct Application {
	windows: Vec<Arc<Window>>,
	event_loop: EventLoop<()>,
	renderer: Option<Renderer>,
}

impl Application {
	pub fn new() -> Application {
		let event_loop = EventLoop::new();

		Application {
			windows: vec![],
			event_loop: event_loop,
			renderer: None
		}
	}

	pub fn open_window(&mut self, dimensions: &Vector2<u32>, name: String) -> Result<Arc<Window>, OsError> {
		let new_window = Arc::new(window::create(dimensions, name, &self.event_loop)?);

		self.windows.push(Arc::clone(&new_window));
		match &mut self.renderer {
			None => self.renderer = Some(Renderer::new(Arc::clone(&new_window))),
			Some(renderer) => renderer.register_window(Arc::clone(&new_window))
		};
		Ok(new_window)
	}

	// Non functional!
	pub fn run(&self) {
		// self.event_loop.run(move |event, _, control_flow| {
		// 	// Wait for events to happen to save on performance
		// 	control_flow.set_wait();

		// 	match event {
		// 		// Close application
		// 		Event::WindowEvent {
		// 			event: WindowEvent::CloseRequested,
		// 			..
		// 		} => {
		// 			control_flow.set_exit();
		// 		}
		// 		// Update application and request redraw
		// 		Event::MainEventsCleared => {

		// 		}
		// 		// Redraw window
		// 		Event::RedrawRequested(_) => {}
		// 		// Default
		// 		_ => (),
		// 	}
		// });
	}
}