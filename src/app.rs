use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;
use winit::event::DeviceEvent;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;
use winit::window::WindowId;
use winit::window::Window;
use crate::render::camera::CameraMove;
use crate::render::draw::draw_ver;
use crate::render::draw::update_camera;
use crate::render::draw::update_vertex_buffer;
use crate::render::wgpu_ctx::*;
use crate::views::home::draw_home;

// 添加 Default 以便App::default()来快速创建App实例
#[derive(Default)]
pub struct App<'window> {
  /// 避免窗口被释放
  #[allow(unused)]
  // 生命唯一的一个窗口实例对象，确保不会多次创建窗口
  window: Option<Arc<Window>>,
  wgpu_ctx: Option<WgpuCtx<'window>>,
  scene: String, // 场景
  mouse_pos: (f64, f64),
  mouse_d_pos: (f64, f64),
  last_time: Option<std::time::Instant>,
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
      // 如果窗口未创建，则创建窗口
      if self.window.is_none() {
        let win_attr = Window::default_attributes().with_title("kidar Engine").with_inner_size(PhysicalSize::new(1600, 900));
        let window = Arc::new(event_loop.create_window(win_attr).expect("Failed to create window"));
        // window.Some(wgpu_ctx);
        let wgpu_ctx = WgpuCtx::new(window.clone());
        self.wgpu_ctx = Some(wgpu_ctx);
        self.window = Some(window);
      }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // 暂停事件
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
          let now = std::time::Instant::now();
          if self.last_time.is_none() {
            self.last_time = Some(now);
          }
          let delta_time = now.duration_since(self.last_time.unwrap()).as_secs_f32();
          self.last_time = Some(now);
          // 处理窗口重绘
          if let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() {
            let size = self.window.as_ref().unwrap().inner_size();
            self.mouse_pos = (size.width as f64/2.0, size.height as f64/2.0);
            wgpu_ctx.camera.set_screen_size(size.width as f32, size.height as f32);

            let vertex_list = draw_home();
            update_vertex_buffer(wgpu_ctx, vertex_list);
            update_camera(wgpu_ctx, delta_time);
            wgpu_ctx.draw();
            // println!("RedrawRequested");
            // draw_ver(wgpu_ctx, vertex_list);
          }

          if let Some(window) = self.window.as_ref() {
            window.request_redraw(); // 请求重绘
          }
        },
        WindowEvent::Focused(focused) => {
          // 处理窗口焦点变化
          if focused {
            // 窗口获得焦点
            println!("Window focused");
          } else {
            // 窗口失去焦点
            println!("Window unfocused");
            if let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() {
              wgpu_ctx.camera.active_move(false);
            }
          }
        }
        WindowEvent::CursorMoved { device_id, position } => {
          // 处理鼠标移动
          self.mouse_d_pos = (position.x - self.mouse_pos.0, position.y - self.mouse_pos.1);
          // println!("Mouse_x {:#?}", &self.mouse_d_pos);
          self.mouse_pos = (position.x, position.y);
        }
        WindowEvent::MouseInput { device_id, state, button } => {
          // TODO: 处理鼠标点击
          if state == winit::event::ElementState::Pressed && button == winit::event::MouseButton::Left {
            println!("Mouse input {:#?}", self.mouse_d_pos);
            if let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() {
                wgpu_ctx.camera.active_move(true);
                self.window.as_ref().unwrap().set_cursor_visible(false);
                self.window.as_ref().unwrap().set_cursor_grab(CursorGrabMode::Locked).unwrap();
            } 
          }
        },
        WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
          // TODO: 处理键盘输入, 按键w、a、s、d 控制相机移动
          // println!("Keyboard input {:#?}， is_synthetic {:#?}", event, is_synthetic);
          if let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() {
            match event.physical_key {
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyW) => {
                  wgpu_ctx.camera.is_forward = event.state == winit::event::ElementState::Pressed;
                },
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyS) => {
                  wgpu_ctx.camera.is_backward = event.state == winit::event::ElementState::Pressed;
                },
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyA) => {
                  wgpu_ctx.camera.is_left = event.state == winit::event::ElementState::Pressed;
                },
                winit::keyboard::PhysicalKey::Code(KeyCode::KeyD) => {
                  wgpu_ctx.camera.is_right = event.state == winit::event::ElementState::Pressed;
                },
                winit::keyboard::PhysicalKey::Code(KeyCode::Space) => {
                  wgpu_ctx.camera.is_up = event.state == winit::event::ElementState::Pressed;
                },
                winit::keyboard::PhysicalKey::Code(KeyCode::ShiftLeft) => {
                  wgpu_ctx.camera.is_down = event.state == winit::event::ElementState::Pressed;
                },
                winit::keyboard::PhysicalKey::Code(KeyCode::Escape) => {
                  self.window.as_ref().unwrap().set_cursor_visible(true);
                  self.window.as_ref().unwrap().set_cursor_grab(CursorGrabMode::None).unwrap();
                  wgpu_ctx.camera.active_move(false);
                }
                _ => {
                  false;
                }
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

    fn device_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
              // println!("MouseMotion: {:#?}", &delta);
              if let Some(wgpu_ctx) = self.wgpu_ctx.as_mut() {
                let now = std::time::Instant::now();
                let delta_time = now.duration_since(self.last_time.unwrap()).as_secs_f32();
                self.last_time = Some(now);
                  wgpu_ctx.camera.look_rotate(delta, delta_time);
              } 
            },
            DeviceEvent::Added => {},
            DeviceEvent::Removed => {},
            DeviceEvent::MouseWheel { delta } => {},
            DeviceEvent::Motion { axis, value } => {},
            DeviceEvent::Button { button, state } => {},
            DeviceEvent::Key(raw_key_event) => {},
        }
    }
    
    
}