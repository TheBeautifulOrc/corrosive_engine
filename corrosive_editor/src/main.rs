use corrosive_engine::core::application::Application;

fn event_handler(event: &winit::event::Event<()>) {
	println!("{:?}", event);
}

fn main() {
	let app = Application::default().with_name("Corrosive Editor");
	_ = app.run(event_handler, false);
}
