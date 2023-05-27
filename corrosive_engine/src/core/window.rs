use winit::{
	dpi::LogicalSize,
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
};

use super::error::EngineError;

pub fn create(
	dimensions: &LogicalSize<u32>,
	name: String,
	event_loop: &EventLoop<()>,
) -> Result<Window, anyhow::Error> {
	if !dimensions_valid(dimensions) {
		Err(EngineError::DimensionError)?
	}

	let window_builder = WindowBuilder::new()
		.with_title(name)
		.with_inner_size(*dimensions);

	let window = window_builder.build(event_loop)?;

	Ok(window)
}

pub fn dimensions_valid<I: std::cmp::PartialOrd<u32>>(dimensions: &LogicalSize<I>) -> bool {
	dimensions.width > 0 && dimensions.height > 0
}
