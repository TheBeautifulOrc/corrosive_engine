use winit::{dpi::LogicalSize, event::WindowEvent, window::Window};

use super::renderer::Renderer;
use super::window;

pub struct Application {
	window: Window,
	renderer: Renderer,
}

impl Application {
	pub async fn new(window: Window) -> Application {
		let renderer = Renderer::new(&window).await;
		Application { window, renderer }
	}

	pub fn window(&self) -> &Window {
		&self.window
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

	pub fn update(&mut self) {

	}

	pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		self.renderer.render()
	}
}
