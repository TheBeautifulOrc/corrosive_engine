use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(
	Copy,
	Clone,
	Debug,
	Pod,
	Zeroable
)]
pub struct Vertex {
	position: [f32; 3],
	color: [f32; 3],
}

impl Vertex {
	const WGPU_ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

	pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
		wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &Self::WGPU_ATTRIBUTES,
		}
	}
}

pub const VERTICES: &[Vertex] = &[
	Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [1.0, 1.0, 1.0] },
	Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [1.0, 0.0, 0.0] },
	Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.0, 1.0, 0.0] },
	Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.0, 0.0, 0.0] },
	Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.0, 0.0, 1.0] },
];

pub const INDICES: &[u16] = &[
	0, 1, 4,
	1, 2, 4,
	2, 3, 4,
];
