#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
  pub position: [f32; 3],
  pub color: [f32; 3],
  pub tex_coords: [f32; 2],
}

unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

pub const VERTEX_LIST: &[Vertex] = &[
  Vertex {
    position: [-0.5, 0.5, 0.0],
    color: [0.5, 0.0, 0.0],
    tex_coords: [0.4131759, 0.00759614],
  },
  Vertex {
    position: [-0.5, -0.5, 0.0],
    color: [0.0, 0.5, 0.0],
    tex_coords: [0.0048659444, 0.43041354],
  },
  Vertex {
    position: [0.0, 0.0, 0.0],
    color: [0.0, 0.0, 0.5],
    tex_coords: [0.28081453, 0.949397],
  },
  Vertex {
    position: [0.5, -0.5, 0.0],
    color: [0.5, 0.0, 0.5],
    tex_coords: [0.28081453, 0.949397],
  },
  Vertex {
    position: [0.5, 0.5, 0.0],
    color: [0.0, 0.5, 0.5],
    tex_coords: [0.28081453, 0.949397],
  }
];

pub const VERTEX_INDEX_LIST: &[u16] = &[
  0, 1, 2,
  2, 3, 4,
  2, 1, 3,
  0, 2, 4
];

pub fn create_vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
  wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &[
      wgpu::VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: wgpu::VertexFormat::Float32x3,
      },
      wgpu::VertexAttribute {
        offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
        shader_location: 1,
        format: wgpu::VertexFormat::Float32x3,
      },
      wgpu::VertexAttribute {
        offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
        shader_location: 2,
        format: wgpu::VertexFormat::Float32x2,
      },
    ],
  }
}