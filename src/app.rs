use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use winit::window::Window;
use crate::render::wgpu_ctx::*;
use crate::views::home::draw_home;

// 添加 Default 以便App::default()来快速创建App实例
#[derive(Default)]
pub struct App<'window> {
  // 生命唯一的一个窗口实例对象，确保不会多次创建窗口
  window: Option<Arc<Window>>,
  wgpu_ctx: Option<WgpuCtx<'window>>,
  mouse_pos: (f64, f64),
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
      // 如果窗口未创建，则创建窗口
      if self.window.is_none() {
        let win_attr = Window::default_attributes().with_title("kidar Engine").with_inner_size(LogicalSize::new(300, 300));
        let window = Arc:: new(event_loop.create_window(win_attr).expect("Failed to create window"));
        // window.Some(wgpu_ctx);
        let wgpu_ctx = WgpuCtx::new(window.clone());
        self.wgpu_ctx = Some(wgpu_ctx);
        self.window = Some(window);
      }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
      match event {
        WindowEvent::Resized(_new_size ) => {
          // 处理窗口大小变化
          if let Some(window) = self.window.as_ref() {
            window.request_redraw(); // 请求重绘
          }
        },
        WindowEvent::RedrawRequested => {
          // 处理窗口重绘
          if let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() {
            let size = self.window.as_ref().unwrap().inner_size();
            // wgpu_ctx.draw();
            let vertex_list = draw_home();
            wgpu_ctx.draw_ver(vertex_list);
          }
        },
        WindowEvent::CursorMoved { device_id, position } => {
          // TODO: 处理鼠标移动
          self.mouse_pos = (position.x, position.y);
          if let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() {
              // wgpu_ctx.update_gpu_buffer(self.mouse_pos);
          } 
        }
        WindowEvent::MouseInput { device_id, state, button } => {
          // TODO: 处理鼠标点击
          if state == winit::event::ElementState::Pressed && button == winit::event::MouseButton::Left {
            println!("Mouse input {:#?}", self.mouse_pos);
            if let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() {
              wgpu_ctx.update_gpu_buffer(self.mouse_pos);
            } 
          }
        },
        WindowEvent::CursorLeft { device_id } => {
          // TODO: 处理鼠标离开窗口
          // println!("Cursor leave window {:#?}", device_id)
        },
        // 处理退出
        WindowEvent::CloseRequested => {
            event_loop.exit();
        }
        _ => ()
      }
    }
}