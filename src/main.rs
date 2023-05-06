use std::thread;
use std::time;
use std::error::Error;

use nalgebra::{Vector2};
use winit::event::{Event, WindowEvent};

mod editor;
mod engine;

use engine::core;



fn main() {
	// Create application, windows and renderer
	let mut app = core::application::Application::new();
	let win = app.open_window(&Vector2::new(800, 600), "Corrosive Editor".to_string()).unwrap();
}
