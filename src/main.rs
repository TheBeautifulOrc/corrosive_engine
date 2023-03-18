use nalgebra::Vector2;

mod engine;
mod editor;

use engine::core::window;

fn main() {
	let (window, event_loop) = window::create(
		&Vector2::new(800, 600),
		"Corrosive Editor".to_string()
	);
}