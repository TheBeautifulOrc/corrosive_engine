use winit::{
	event_loop::EventLoop,
	window::{WindowBuilder, Window}
};
use nalgebra::Vector2;

pub fn create(dimensions: &Vector2<u32>, name: String) -> (Window, EventLoop<()>) {
	let event_loop = EventLoop::new();

	let window_builder = WindowBuilder::new()
		.with_title(name)
		.with_inner_size(winit::dpi::LogicalSize::new(dimensions.x, dimensions.y));

	let window = window_builder.build(&event_loop).expect("failed to create window");

	(window, event_loop)
}