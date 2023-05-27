use std::default::Default;

use winit::{
	dpi::LogicalSize,
	event::{Event, WindowEvent},
	event_loop::EventLoop,
	window::Window,
};

use pollster;

use super::error;
use super::renderer::Renderer;
use super::window;

pub struct Application {
	event_loop: Option<EventLoop<()>>,
	window: Window,
	renderer: Renderer,
}

impl Application {
	pub async fn new(event_loop: EventLoop<()>, window: Window) -> Application {
		let renderer = Renderer::new(&window).await;
		Application {
			event_loop: Some(event_loop),
			window,
			renderer,
		}
	}
}

impl Default for Application {
	fn default() -> Self {
		let event_loop = EventLoop::new();
		let window = window::create(
			&LogicalSize::new(800, 600),
			"Corrosive Application".to_string(),
			&event_loop,
		)
		.unwrap();
		pollster::block_on(Self::new(event_loop, window))
	}
}

impl Application {
	pub fn rename(&mut self, new_title: &str) {
		self.window.set_title(new_title);
	}

	pub fn with_name(mut self, new_title: &str) -> Self {
		self.rename(new_title);
		self
	}


	pub fn processed(&self, _event: &WindowEvent) -> bool {
		false
	}

	pub fn resize(&mut self, new_dimensions: &LogicalSize<u32>) {
		if !window::dimensions_valid(new_dimensions) {
			return;
		}
		self.window.set_inner_size(*new_dimensions);
		self.renderer.resize(new_dimensions);
	}

	pub fn update(&mut self) {}

	pub fn run(mut self) -> Result<(), anyhow::Error> {
		// Make sure the event loop is present, otherwise terminate
		if self.event_loop.is_none() {
			return Err(error::EngineError::EventLoopElapsed.into());
		}

		// Remove event loop from application to correctly call run function
		let event_loop = self.event_loop.take().unwrap();
		// Run event loop
		event_loop.run(move |event, _, control_flow| {
			control_flow.set_wait();

			match event {
				Event::WindowEvent {
					ref event,
					window_id,
				} if window_id == self.window.id() => {
					if !self.processed(event) {
						match event {
							WindowEvent::CloseRequested => {
								control_flow.set_exit();
							}
							WindowEvent::Resized(physical_size) => {
								let ls: LogicalSize<u32> =
									physical_size.to_logical(self.window.scale_factor());
								self.resize(&ls);
							}
							WindowEvent::ScaleFactorChanged {
								scale_factor,
								new_inner_size,
							} => {
								let ls: LogicalSize<u32> = new_inner_size.to_logical(*scale_factor);
								self.resize(&ls);
							}
							_ => {}
						}
					}
				}
				Event::MainEventsCleared => self.window.request_redraw(),
				Event::RedrawRequested(window_id) if window_id == self.window.id() => {
					self.update();
					match self.renderer.render() {
						Ok(_) => {}
						// Reconfigure surface if it is lost
						Err(wgpu::SurfaceError::Lost) => {
							let win = &self.window;
							let current_size: LogicalSize<u32> =
								win.inner_size().to_logical(win.scale_factor());
							self.resize(&current_size);
						}
						// Quit if system memory is depleted
						Err(wgpu::SurfaceError::OutOfMemory) => {
							control_flow.set_exit();
						}
						// Print other errors (should resolve by themselves)
						Err(e) => {
							eprintln!("{:?}", e);
						}
					}
				}
				_ => {}
			}
		});
	}
}
