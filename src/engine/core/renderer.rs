use std::sync::Arc;
use winit::{
	window::Window,
	dpi::PhysicalSize
};

pub struct Renderer {
	window: Arc<Window>,
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
}

impl Renderer {
	pub fn new(window: Arc<Window>) -> Renderer {
		let instance = wgpu::Instance::new(
			wgpu::InstanceDescriptor {
				backends: wgpu::Backends::all(),
				dx12_shader_compiler: Default::default(),
			}
		);
		let raw_window = window.as_ref();
		let surface = unsafe { instance.create_surface(raw_window) }.unwrap();

		let adapter = instance.request_adapter(
			&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::HighPerformance,
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			},
		);

		todo!()
	}

	pub fn register_window(&mut self, window: Arc<Window>) {
		todo!()
	}

}