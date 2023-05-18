use pollster;

use winit::{
	dpi::LogicalSize,
	event::{Event, WindowEvent},
	event_loop::EventLoop,
};

use corrosive_engine::core::{application::Application, window};

fn main() {
	// Prepare event loop
	let event_loop = EventLoop::new();
	// Create windows
	let window = window::create(
		&LogicalSize::new(800, 600),
		"Corrosive Editor".to_string(),
		&event_loop,
	)
	.unwrap();
	let mut app = pollster::block_on(Application::new(window));

	// Run event loop
	event_loop.run(move |event, _, control_flow| {
		control_flow.set_wait();

		match event {
			Event::WindowEvent {
				ref event,
				window_id,
			} if window_id == app.window().id() => {
				if !app.processed(event) {
					match event {
						WindowEvent::CloseRequested => {
							control_flow.set_exit();
						}
						WindowEvent::Resized(physical_size) => {
							let ls: LogicalSize<u32> =
								physical_size.to_logical(app.window().scale_factor());
							app.resize(&ls);
						}
						WindowEvent::ScaleFactorChanged {
							scale_factor,
							new_inner_size,
						} => {
							let ls: LogicalSize<u32> = new_inner_size.to_logical(*scale_factor);
							app.resize(&ls);
						}
						_ => {}
					}
				}
			}
			Event::MainEventsCleared => app.window().request_redraw(),
			Event::RedrawRequested(window_id) if window_id == app.window().id() => {
				app.update();
				match app.render() {
					Ok(_) => {}
					// Reconfigure surface if it is lost
					Err(wgpu::SurfaceError::Lost) => {
						let win = app.window();
						let current_size: LogicalSize<u32> =
							win.inner_size().to_logical(win.scale_factor());
						app.resize(&current_size);
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
