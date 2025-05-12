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
      front_face: FrontFace::Cw, // Ccw:逆时针顶点顺序为正面（默认）, Cw:顺时针顶点顺序为正面
      cull_mode: Some(Face::Back), // 背面剔除,
      unclipped_depth: false, // 是否禁用近/远平面的深度裁剪, 默认false（启用裁剪）
      polygon_mode: PolygonMode::Fill, // 设置为线框模式， 片源着色器绘制类型
      conservative: false, // 是否启用保守光栅化
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