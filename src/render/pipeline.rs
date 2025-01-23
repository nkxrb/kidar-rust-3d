use crate::render::vertex::*;
use wgpu::*;

pub fn create_pipeline(device: &Device, texture_format: TextureFormat, bind_group_layout: &BindGroupLayout) -> RenderPipeline {
  let shader: ShaderModule = device.create_shader_module(ShaderModuleDescriptor {
    label: Some("Shader"),
    source: ShaderSource::Wgsl(include_str!("../template/shader.wgsl").into()),
  });

  let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
    label: Some("Render Pipeline Layout"),
    bind_group_layouts: &[bind_group_layout],
    push_constant_ranges: &[],
  });

  device.create_render_pipeline(&RenderPipelineDescriptor {
    label: Some("Render Pipeline"),
    layout: Some(&render_pipeline_layout),
    vertex: VertexState {
      module: &shader,
      entry_point: Some("vs_main"),
      buffers: &[
        create_vertex_buffer_layout()
      ],
      compilation_options: Default::default(),
    },
    fragment: Some(FragmentState {
      module: &shader,
      entry_point: Some("fs_main"),
      targets: &[Some(texture_format.into())],
      compilation_options: Default::default(),
    }),
    primitive: PrimitiveState {
      topology: PrimitiveTopology::TriangleList,
      strip_index_format: None,
      front_face: FrontFace::Ccw,
      cull_mode: None,
      unclipped_depth: false,
      polygon_mode: PolygonMode::Fill, // 设置为线框模式， 片源着色器绘制类型
      conservative: false,
    },
    depth_stencil: None,
    multisample: MultisampleState {
      count: 1,
      mask: !0,
      alpha_to_coverage_enabled: false,
    },
    multiview: None,
    cache: None,
  })
}