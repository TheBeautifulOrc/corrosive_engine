use std::option::Option;
use std::sync::Arc;

use vulkano::{
	device::{
		physical::PhysicalDevice,
		Device,
		DeviceCreateInfo,
		Queue,
		QueueCreateInfo,
	},
	instance::{Instance, InstanceCreateInfo},
	VulkanLibrary,
};

#[derive(Debug)]
pub struct Renderer {
	instance: Arc<Instance>,
	physical_devices: Vec<Arc<PhysicalDevice>>,
	active_physical_device: Arc<PhysicalDevice>,
	logical_device: Arc<Device>,
	queues: Vec<Arc<Queue>>,
}

impl Renderer {
	// TODO: implement support for multiple physical devices?
	// TODO: add support for device extensions and features
	pub fn new(
		requested_physical_device_index: usize,
		requested_queues: Option<Vec<QueueCreateInfo>>,
	) -> Result<Self, String> {
		let library = Renderer::find_vulkan()?;
		let instance = Renderer::create_instance(library)?;
		let (physical_devices, active_physical_device) =
			Renderer::get_physical(&instance, requested_physical_device_index)?;

		let device_create_info = match requested_queues {
			Some(n) => DeviceCreateInfo {
				queue_create_infos: n,
				..Default::default()
			},
			None => {
				let q_family_indices = Renderer::select_queue_families(&active_physical_device)?;
				let mut q_create_infos: Vec<QueueCreateInfo> = vec![];
				for index in q_family_indices {
					q_create_infos.push(QueueCreateInfo {
						queue_family_index: index,
						..Default::default()
					})
				}
				DeviceCreateInfo {
					queue_create_infos: q_create_infos,
					..Default::default()
				}
			}
		};

		let (device, queues) =
			Renderer::device_and_queues(&active_physical_device, device_create_info)?;

		Ok(Renderer {
			instance: instance,
			physical_devices: physical_devices,
			active_physical_device: active_physical_device,
			logical_device: device,
			queues: queues,
		})
	}

	/// Locate valid Vulkan implementation
	fn find_vulkan() -> Result<Arc<VulkanLibrary>, String> {
		let library = match VulkanLibrary::new() {
			Ok(n) => n,
			Err(e) => {
				return Err("failed to locate Vulkan library/DLL".to_string());
			}
		};
		Ok(library)
	}

	/// Create a Vulkano instance, the API entrance point
	fn create_instance(library: Arc<VulkanLibrary>) -> Result<Arc<Instance>, String> {
		let instance = match Instance::new(library, InstanceCreateInfo::default()) {
			Ok(n) => n,
			Err(e) => {
				return Err("failed to create Vulkan instance".to_string());
			}
		};
		Ok(instance)
	}

	/// Get list of physical devices and mark the first one as active
	fn get_physical(
		instance: &Arc<Instance>,
		requested_active_index: usize,
	) -> Result<(Vec<Arc<PhysicalDevice>>, Arc<PhysicalDevice>), String> {
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
		// Default to using the first physical device (can be changed manually later on)
		let active_physical_device = match physical_devices.get(requested_active_index) {
			Some(n) => n.clone(),
			None => {
				return Err("requested physical device does not exist".to_string());
			}
		};

		Ok((physical_devices, active_physical_device))
	}

	/// Default selection strategy for necessary queue families
	fn select_queue_families(active_physical_device: &Arc<PhysicalDevice>) -> Result<Vec<u32>, String> {
		// The indices of all queue families we want to include in our application
		let mut requested_queue_families: Vec<u32> = vec![];
		// All the queue family features that we have covered with our currently requested queue families
		let mut covered_flags = [false, false, false, false];

		for (f_index, family) in active_physical_device
			.queue_family_properties()
			.iter()
			.enumerate()
		{
			let q_flags = family.queue_flags;
			let q_flags = [
				q_flags.graphics,
				q_flags.compute,
				q_flags.transfer,
				q_flags.sparse_binding,
			];
			// Should we add the current family to the list of requested ones
			let mut shall_add = false;
			// Iterate through all queue family features
			for (covered, current) in covered_flags.iter_mut().zip(q_flags.iter()) {
				// The current queue family shall be added if it provides features that were not covered before
				if !*covered && *current {
					*covered = true;
					shall_add = true;
				}
			}
			if shall_add { requested_queue_families.push(f_index as u32); }
		}

		let mut covered_all = true;
		for f in covered_flags.iter() {
			covered_all &= *f;
		}

		match covered_all {
			true => Ok(requested_queue_families),
			false => {
				return Err("Could not find suitable queue family for each feature".to_string());
			}
		}

	}

	/// Create Vulkano device representation object and get associated queues
	fn device_and_queues(
		active_physical_device: &Arc<PhysicalDevice>,
		device_create_info: DeviceCreateInfo,
	) -> Result<(Arc<Device>, Vec<Arc<Queue>>), String> {
		let (device, mut queue_iter) = match Device::new(
			active_physical_device.clone(),
			device_create_info
		) {
			Ok(n) => n,
			Err(e) => {
				return Err("failed to crate device object".to_string());
			}
		};
		let mut queues: Vec<Arc<Queue>> = vec![];
		for queue in queue_iter {
			queues.push(queue);
		}
		Ok((device, queues))
	}

	/// Select queue
	/// TODO: Come up with strategy on how many queues to use and why
	fn select_queue(queues: &Vec<Arc<Queue>>) -> Result<Arc<Queue>, String> {
		let queue = match queues.len() {
			0 => {
				return Err("selected queue family empty".to_string());
			}
			_ => queues[0].clone(),
		};
		Ok(queue)
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
				println!(
					"\t\tSparse Binding:\t{:?}",
					family.queue_flags.sparse_binding
				);
				println!("\tQueue count: {:?}", family.queue_count);
			}
			println!();
		}
	}
}

impl Default for Renderer {
	fn default() -> Self {
		match Self::new(
			0,
			None,
		) {
			Ok(n) => n,
			Err(e) => panic!("{}", e),
		}
	}
}
