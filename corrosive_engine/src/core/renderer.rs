use winit::{dpi::LogicalSize, window::Window};

pub struct Renderer {
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
}

impl Renderer {
	pub async fn new(window: &Window) -> Renderer {
		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::all(),
			dx12_shader_compiler: Default::default(),
		});
		let surface = unsafe { instance.create_surface(window) }.unwrap();

		let adapter: wgpu::Adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::HighPerformance,
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			})
			.await
			.unwrap();

		let (device, queue): (wgpu::Device, wgpu::Queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					features: wgpu::Features::empty(),
					limits: if cfg!(target_arch = "wasm32") {
						wgpu::Limits::downlevel_webgl2_defaults()
					} else {
						wgpu::Limits::default()
					},
					label: None,
				},
				None,
			)
			.await
			.unwrap();

		let surface_caps = surface.get_capabilities(&adapter);
		let surface_format = surface_caps
			.formats
			.iter()
			.copied()
			.filter(|f| f.is_srgb())
			.next()
			.unwrap_or(surface_caps.formats[0]);
		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: window.inner_size().width,
			height: window.inner_size().height,
			present_mode: surface_caps.present_modes[0],
			alpha_mode: surface_caps.alpha_modes[0],
			view_formats: vec![],
		};
		surface.configure(&device, &config);

		Self {
			surface,
			device,
			queue,
			config,
		}
	}

	pub fn resize(&mut self, new_dimensions: &LogicalSize<u32>) {
		self.config.width = new_dimensions.width;
		self.config.height = new_dimensions.height;

		self.surface.configure(&self.device, &self.config);
	}
}
