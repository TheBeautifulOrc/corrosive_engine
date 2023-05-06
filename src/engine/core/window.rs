use nalgebra::Vector2;

use winit::{
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
	error::OsError
};

pub fn create(dimensions: &Vector2<u32>, name: String, event_loop: &EventLoop<()>) -> Result<Window, OsError> {

	let window_builder = WindowBuilder::new()
		.with_title(name)
		.with_inner_size(winit::dpi::LogicalSize::new(dimensions.x, dimensions.y));

	let window = window_builder
		.build(event_loop)?;

	Ok(window)
}
