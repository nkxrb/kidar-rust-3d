use nalgebra::{Matrix4, Point3, Vector3};
use wgpu::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CameraUniform {
  proj: Matrix4<f32>,
  view: Matrix4<f32>,
}

unsafe impl bytemuck::Zeroable for CameraUniform {}
unsafe impl bytemuck::Pod for CameraUniform {}

pub struct Camera {
  position: Vector3<f32>, // 相机位置
  target: Vector3<f32>, // 相机目标
  up: Vector3<f32>, // 相机上方向
  fov: f32, // 视场角
  aspect: f32, // 宽高比
  near: f32, // 近裁剪面
  far: f32, // 远裁剪面
}

impl Camera {
  pub fn new(position: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>, fov: f32, aspect: f32, near: f32, far: f32) -> Self {
    Self {
      position,
      target,
      up,
      fov,
      aspect,
      near,
      far,
    }
  }

  pub fn view_matrix(&self) -> Matrix4<f32> {
    Matrix4::look_at_rh(&Point3::from(self.position), &Point3::from(self.target), &self.up)
  }

  pub fn projection_matrix(&self) -> Matrix4<f32> {
    let f = 1.0 / (self.fov * 0.5).tan();
    let aspect = self.aspect;
    let z_range = self.far - self.near;
    let z_scale = self.far / (self.far - self.near);
    let z_offset = -(self.far * self.near) / z_range;
    let projection = Matrix4::new(
      f / aspect, 0.0, 0.0, 0.0,
      0.0, f, 0.0, 0.0,
      0.0, 0.0, z_scale, -1.0,
      0.0, 0.0, z_offset, 0.0,
    );
    projection
  } 

  pub fn uniform_obj(&self) -> CameraUniform  {
    CameraUniform {
      proj: self.projection_matrix(),
      view: self.view_matrix(),
    }
  }

  pub fn bind_group(&self, device: &Device, bind_group_layout: &BindGroupLayout, uniform_buffer: &Buffer) -> BindGroup {

    // 创建 Bind Group
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            },
        ],
        label: Some("View Matrix Bind Group"),
    })
    // bind_group
  }
}