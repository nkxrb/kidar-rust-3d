use wgpu::*;

use super::{camera::Camera, vertex::Vertex, wgpu_ctx::WgpuCtx};

pub fn create_depth_texture(ctx: &mut WgpuCtx, label: &str) -> (wgpu::Texture, wgpu::TextureView) {
  let depth_texture_desc = wgpu::TextureDescriptor {
      label: Some(label),
      size: wgpu::Extent3d {
          width: ctx.vw,
          height: ctx.vh,
          depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Depth32Float,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
  };

  let texture = ctx.device.create_texture(&depth_texture_desc);
  let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
  (texture, view)
}


// pub fn get_depth_view(ctx: &mut WgpuCtx) {
//   let mut dept_view_desc = TextureViewDescriptor::default();
//   dept_view_desc.format = Some(TextureFormat::Depth32Float);
//   return  dept_view_desc;
// }
pub fn update_vertex_buffer(ctx: &mut WgpuCtx, vertex_list: Vec<Vertex>) {
  ctx.queue.write_buffer(&ctx.vertex_buffer, 0, bytemuck::cast_slice(&vertex_list));
  ctx.vertex_len = vertex_list.len() as u32;
}

pub fn draw_ver(ctx: &mut WgpuCtx, vertex_list: Vec<Vertex>) {
  ctx.queue.write_buffer(&ctx.vertex_buffer, 0, bytemuck::cast_slice(&vertex_list));
  ctx.vertex_len = vertex_list.len() as u32;

  let frame = ctx.surface.get_current_texture().unwrap();
  // 设置纹理
  let view = frame.texture.create_view(&TextureViewDescriptor::default());
  // println!("WgpuCtx::draw: {:?}", view);
  let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor { label: None });
  let depth_view = create_depth_texture(ctx, "depth_texture").1;
  // 此处使用作用域，将pass限制在一定范围内，出作用域后会自动调用drop清理资源。
  {
    let mut r_pass = encoder.begin_render_pass(&RenderPassDescriptor {
      label: None,
      depth_stencil_attachment: Some(RenderPassDepthStencilAttachment { 
          view: &depth_view,
          depth_ops: Some(Operations {
            load: LoadOp::Clear(1.0),
            store: StoreOp::Store,
          }),
          stencil_ops: None,
        }),
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
    // println!("r_pass: {:#?}", &ctx.bind_group.into());
    r_pass.set_pipeline(&ctx.render_pipeline);
    r_pass.set_bind_group(0, &ctx.bind_group, &[]);
    r_pass.set_vertex_buffer(0, ctx.vertex_buffer.slice(..));
    r_pass.draw(0..vertex_list.len() as u32, 0..1);
  }

  // 上面的pass结束后，才能调用finish
  ctx.queue.submit(Some(encoder.finish())); // 提交命令到GPU
  frame.present(); // 替换当前帧画面，显示最新的图像
}

pub fn update_camera(ctx: &mut WgpuCtx, dt:f32) {
  ctx.camera.update(dt);
  ctx.queue.write_buffer(&ctx.vertex_uniform_buffer, 0, bytemuck::cast_slice(&[ctx.camera.uniform_obj()]));
}