use wgpu::*;

use super::{camera::Camera, vertex::Vertex, wgpu_ctx::WgpuCtx};

pub fn draw_ver(ctx: &mut WgpuCtx, vertex_list: Vec<Vertex>) {
  ctx.queue.write_buffer(&ctx.vertex_buffer, 0, bytemuck::cast_slice(&vertex_list));
  ctx.vertex_len = vertex_list.len() as u32;

  let frame = ctx.surface.get_current_texture().unwrap();
  // 设置纹理
  let view = frame.texture.create_view(&TextureViewDescriptor::default());
  // println!("WgpuCtx::draw: {:?}", view);
  let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor { label: None });

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

pub fn update_camera(ctx: &mut WgpuCtx) {
  ctx.queue.write_buffer(&ctx.vertex_uniform_buffer, 0, bytemuck::cast_slice(&[ctx.camera.uniform_obj()]));
}