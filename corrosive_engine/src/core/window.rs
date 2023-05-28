use winit::{
	dpi::LogicalSize,
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
};

use super::error::EngineError;

/// Initializes new `winit::window::Window` object.
///
/// # Arguments
///
/// - `dimensions` - Dimensions of the window. Make sure these are greater than 0
/// - `name` - Window title
/// - `event_loop` - Event loop that will handle this windows events
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

/// Checks if the dimensions of a `winit::LogicalSize` object are valid (> 0).
///
/// Returns `true` if so.
///
/// # Arguments
///
/// - `dimensions` - The dimensions to check for validity
///
/// # Example
///
/// ```
/// use winit::dpi::LogicalSize;
/// use corrosive_engine::core::window;
///
/// let valid_dimensions = LogicalSize::<u32>::new(800, 600);
/// let invalid_dimensions = LogicalSize::<u32>::new(0, 600);
///
/// assert!(window::dimensions_valid(&valid_dimensions));
/// assert!(!window::dimensions_valid(&invalid_dimensions));
/// ```
pub fn dimensions_valid<I: std::cmp::PartialOrd<u32>>(dimensions: &LogicalSize<I>) -> bool {
	dimensions.width > 0 && dimensions.height > 0
}
