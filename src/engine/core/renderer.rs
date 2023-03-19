use std::option::Option;
use std::sync::Arc;

use vulkano::{
	device::physical::PhysicalDevice,
	instance::{Instance, InstanceCreateInfo},
	VulkanLibrary,
};

#[derive(Debug)]
pub struct Renderer {
	pub instance: Arc<Instance>,
	pub physical_devices: Vec<Arc<PhysicalDevice>>,
	active_physical_device: usize,
}

impl Renderer {
	pub fn new() -> Result<Renderer, String> {
		// Locate valid Vulkan implementation
		let library = match VulkanLibrary::new() {
			Ok(n) => n,
			Err(e) => {
				return Err("failed to locate Vulkan library/DLL".to_string());
			}
		};

		// Create API entrance point
		let instance = match Instance::new(library, InstanceCreateInfo::default()) {
			Ok(n) => n,
			Err(e) => {
				return Err("failed to create Vulkan instance".to_string());
			}
		};

		// Get list of physical devices
		let physical = match instance.enumerate_physical_devices() {
			Ok(n) => n,
			Err(e) => {
				return Err("failed to enumerate physical devices".to_string());
			}
		};
		// Push physical device references into vector and find one that is graphical
		let mut physical_devices: Vec<Arc<PhysicalDevice>> = vec![];
		for physical_device in physical {
			physical_devices.push(physical_device);
		}
		// Default to using the first physical device (can be changed manually)
		let active_physical_device = 0;

		Ok(Renderer {
			instance: instance,
			physical_devices: physical_devices,
			active_physical_device: 0,
		})
	}

	#[cfg(debug_assertions)]
	pub fn print_physical_devices(&self) {
		for (d_index, device) in self.physical_devices.iter().enumerate() {
			println!("{:?}. {:?}:", d_index + 1, device.properties().device_name);
			for (q_index, family) in device.queue_family_properties().iter().enumerate() {
				println!("\t{:?}. Queue:", q_index + 1);
				println!("\t\tGraphics:\t{:?}", family.queue_flags.graphics);
				println!("\t\tCompute:\t{:?}", family.queue_flags.compute);
				println!("\t\tTransfer:\t{:?}", family.queue_flags.transfer);
				println!("\t\tSparse:\t{:?}", family.queue_flags.sparse_binding);
				println!("\tQueue count: {:?}", family.queue_count);
			}
			println!();
		}
	}

	// fn initialization_after_physical_device() ->
}
