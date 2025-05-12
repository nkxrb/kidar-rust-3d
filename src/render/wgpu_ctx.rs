use std::sync::Arc;

use nalgebra::Vector3;
use util::{BufferInitDescriptor, DeviceExt};
use winit::{dpi::PhysicalSize, window::Window};
use wgpu::*;

use crate::render::{camera::Camera, pipeline::create_pipeline, vertex::*};

use super::camera::CameraMove;

pub struct WgpuCtx<'window> {
  pub vw: u32, // 屏幕高度
  pub vh: u32, // 屏幕宽度
  pub surface: Surface<'window>,
  pub device: Device,
  pub queue: Queue,
  pub surface_config: SurfaceConfiguration,
  pub adapter: Adapter,
  pub render_pipeline: RenderPipeline,
  pub vertex_buffer: Buffer,
  pub vertex_index_buffer: Buffer,
  pub vertex_uniform_buffer: Buffer,
  pub bind_group: BindGroup,
  pub camera: Camera,
  pub vertex_len: u32
}

impl<'window> WgpuCtx<'window> {

  pub async fn new_async(window: Arc<Window>) -> Self {
    // 构建wgpu上下文
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

    // 获取GPU的逻辑设备、命令队列
    let (device, queue) = adapter.request_device(&DeviceDescriptor {
      label: None,
      trace: Trace::Off,
      required_features: wgpu::Features::POLYGON_MODE_LINE,
      required_limits: wgpu::Limits::default(),
      memory_hints: Default::default(),
    }).await.expect("Failed to create device");

    let window_size = window.inner_size();
    let width = window_size.width;
    let height = window_size.height;
    println!("width: {}, height: {}", width, height);
    // 设置表面配置对象
    let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
    // 将表面配置对象应用到表面
    surface.configure(&device, &surface_config);

    // 创建 Bind Group Layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
        label: Some("Uniform Bind Group Layout"),
    });

    // 创建渲染管线
    let render_pipeline = create_pipeline(&device, surface_config.format, &bind_group_layout);
    // 创建顶点缓存器
    let vertex_buffer = device.create_buffer(&BufferDescriptor {
      label: None,
      size: 32000, // 当前一个顶点是32个字节，这里最大支持1000个顶点
      usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
      mapped_at_creation: false
    });
    // 创建顶点索引缓存器
    let vertex_index_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&VERTEX_INDEX_LIST),
      usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
    });

    let screen_width = window_size.width as f32;
    let screen_height = window_size.height as f32;
    // 创建相机
    let camera = Camera::new(
      Vector3::new(5100.0, 2200.0, 0.0), // 相机位置
      Vector3::new(5100.0, 2200.0, 2200.0), // 观察点
      Vector3::new(0.0, 1.0, 0.0), // 相机朝上的方向
      45.0_f32.to_radians(), // 相机的视野角度
      screen_width, // 屏幕宽度
      screen_height, // 屏幕高度,
      0.1, // 最近的可见距离
      100.0, // 最远的可见距离
    );
    println!("screen_width: {}, screen_height: {}", screen_width, screen_height);
    let vertex_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
      label: Some("uniform_buffer"),
      contents: bytemuck::cast_slice(&[camera.uniform_obj()]),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let bind_group = camera.bind_group(&device, &bind_group_layout, &vertex_uniform_buffer);

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
        vertex_index_buffer,
        vertex_uniform_buffer,
        bind_group,
        camera,
        vertex_len: 0
      };
  }

  pub fn new (window: Arc<Window>) -> Self {
    pollster::block_on(Self::new_async(window))
  }
}

impl<'window> WgpuCtx<'window> {
  pub fn draw(&mut self) {
    let frame = self.surface.get_current_texture().unwrap();
    // 设置纹理
    let view = frame.texture.create_view(&TextureViewDescriptor::default());
    // println!("WgpuCtx::draw: {:?}", view);
    let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: None });

    // 此处使用作用域，将pass限制在一定范围内，出作用域后会自动调用drop清理资源。
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
            load: LoadOp::Clear(Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 }),
            store: StoreOp::Store,
          },
        })]
      });
      // println!("r_pass: {:#?}", &self.bind_group.into());
      r_pass.set_pipeline(&self.render_pipeline);
      r_pass.set_bind_group(0, &self.bind_group, &[]);
      r_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
      r_pass.draw(0..self.vertex_len, 0..1);
      // r_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
      // r_pass.set_index_buffer(self.vertex_index_buffer.slice(..), IndexFormat::Uint16);
      // r_pass.draw_indexed(0..VERTEX_INDEX_LIST.len() as u32, 0, 0..1);
      // r_pass.draw(0..VERTEX_LIST.len() as u32, 0..1);
    }

    // 上面的pass结束后，才能调用finish
    self.queue.submit(Some(encoder.finish())); // 提交命令到GPU
    frame.present(); // 替换当前帧画面，显示最新的图像
  }

  pub fn update_gpu_buffer(&mut self, mouse_pos: (f64, f64)) {
    // 小数据更新，直接更新的是GPU内部的buffer
    self.camera.set_position(Vector3::new(mouse_pos.0 as f32, mouse_pos.1 as f32, 2.0));
    self.queue.write_buffer(&self.vertex_uniform_buffer, 0, bytemuck::cast_slice(&[self.camera.uniform_obj()]));
    self.draw();
  } 

  pub fn move_camera_buffer(&mut self, move_direction: CameraMove) {
    // 小数据更新，直接更新的是GPU内部的buffer
    match move_direction {
      CameraMove::Forward => {
        self.camera.move_forward(10.0);
      },
      CameraMove::Backward => {
        self.camera.move_backward(10.0);
      },
      CameraMove::Left => {
        self.camera.move_left(10.0);
      },
      CameraMove::Right => {
        self.camera.move_right(10.0);
      }
      CameraMove::Up => {
        self.camera.look_up(10.0);
      },
      CameraMove::Down => {
        self.camera.look_down(10.0);
      },
      CameraMove::None => {
        return;
      },
    }
    // self.camera.set_position(Vector3::new(mouse_pos.0 as f32, mouse_pos.1 as f32, 2.0));
    self.queue.write_buffer(&self.vertex_uniform_buffer, 0, bytemuck::cast_slice(&[self.camera.uniform_obj()]));
    self.draw();
  } 

  pub fn update_uniform_buffer(&mut self, size: PhysicalSize<u32>) {
    // 根据窗口大小，更新projection矩阵
    // let projection = create_projection(self.vw, self.vh);
  }
}


