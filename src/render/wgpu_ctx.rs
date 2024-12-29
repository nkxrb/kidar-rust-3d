use std::sync::Arc;

use util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;
use wgpu::*;

use crate::render::vertex::*;

pub struct WgpuCtx<'window> {
  vw: u32, // 屏幕高度
  vh: u32, // 屏幕宽度
  surface: Surface<'window>,
  device: Device,
  queue: Queue,
  surface_config: SurfaceConfiguration,
  adapter: Adapter,
  render_pipeline: RenderPipeline,
  vertex_buffer: Buffer,
  vertex_index_buffer: Buffer,
}

impl<'window> WgpuCtx<'window> {

  pub async fn new_async(window: Arc<Window>) -> Self {
    // 构建wgpu上下文
    println!("WgpuCtx::new");
    // 创建一个wgpu实例
    let instance = wgpu::Instance::default();
    // 初始化一个画布表面
    let surface = instance.create_surface(window.clone()).unwrap();
    // 获取适配器
    let adapter = instance.request_adapter(&RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      compatible_surface: Some(&surface),
      force_fallback_adapter: false,
    }).await.expect("Failed to find an appropriate adapter");

    // 获取逻辑设备、命令队列
    let (device, queue) = adapter.request_device(&DeviceDescriptor {
      label: None,
      required_features: wgpu::Features::empty(),
      required_limits: wgpu::Limits::default(),
      memory_hints: Default::default(),
    }, None).await.expect("Failed to create device");

    let window_size = window.inner_size();
    let width = window_size.width;
    let height = window_size.height;
    println!("width: {}, height: {}", width, height);
    // 设置表面配置对象
    let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
    // 将表面配置对象应用到表面
    surface.configure(&device, &surface_config);

    // 创建渲染管线
    let render_pipeline = create_pipeline(&device, surface_config.format);
    // 创建顶点缓存器
    // let vertex_buffer = create_vertex_buffer(&device, &VERTEX_LIST);
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&VERTEX_LIST),
      usage: BufferUsages::VERTEX,
    });
    // 创建顶点索引缓存器
    let vertex_index_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&VERTEX_INDEX_LIST),
      usage: BufferUsages::INDEX
    });

    return WgpuCtx {
        vw: width,
        vh: height,
        surface: surface,
        device: device,
        queue: queue,
        surface_config: surface_config,
        adapter: adapter,
        render_pipeline,
        vertex_buffer,
        vertex_index_buffer
      };
  }

  pub fn new (window: Arc<Window>) -> Self {
    pollster::block_on(Self::new_async(window))
  }

  pub fn draw(&mut self) {
    // 画图
    let frame = self.surface.get_current_texture().unwrap();
    let view = frame.texture.create_view(&TextureViewDescriptor::default());
    // println!("WgpuCtx::draw: {:?}", view);
    let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: None });
    {
      let mut r_pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: None,
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        color_attachments: &[Some(RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: Operations {
            load: LoadOp::Clear(Color {
              r: 0.1,
              g: 0.2,
              b: 0.3,
              a: 1.0,
            }),
            store: StoreOp::Store,
          },
        })]
      });
      r_pass.set_pipeline(&self.render_pipeline);
      r_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
      r_pass.set_index_buffer(self.vertex_index_buffer.slice(..), IndexFormat::Uint16);
      r_pass.draw_indexed(0..VERTEX_INDEX_LIST.len() as u32, 0, 0..1);
      r_pass.draw(0..VERTEX_LIST.len() as u32, 0..1);
    }
    self.queue.submit(Some(encoder.finish()));
    frame.present();
  }

}

fn create_pipeline(device: &Device, textureFormat: TextureFormat) -> RenderPipeline {
  let shader = device.create_shader_module(ShaderModuleDescriptor {
    label: Some("Shader"),
    source: ShaderSource::Wgsl(include_str!("../template/shader.wgsl").into()),
  });

  let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
    label: Some("Render Pipeline Layout"),
    bind_group_layouts: &[],
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
      targets: &[Some(textureFormat.into())],
      compilation_options: Default::default(),
    }),
    primitive: PrimitiveState {
      topology: PrimitiveTopology::TriangleList,
      strip_index_format: None,
      front_face: FrontFace::Ccw,
      cull_mode: None,
      unclipped_depth: false,
      polygon_mode: PolygonMode::Fill,
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

fn create_vertex_buffer(device: &Device, vertices: &[Vertex]) -> Buffer {
  let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
    label: Some("Vertex Buffer"),
    contents: bytemuck::cast_slice(vertices),
    usage: BufferUsages::VERTEX,
  });
  return vertex_buffer;
  // queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(vertices));
}




