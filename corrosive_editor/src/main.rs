use corrosive_engine::core::{application::Application};

fn main() {
	let app = Application::default().with_name("Corrosive Editor");
	_ = app.run();
}
