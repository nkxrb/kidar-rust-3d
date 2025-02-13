use nalgebra::{Matrix4, Point3, Vector3};
use wgpu::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CameraUniform {
  proj: Matrix4<f32>,
  view: Matrix4<f32>,
  model: Matrix4<f32>,
}

unsafe impl bytemuck::Zeroable for CameraUniform {}
unsafe impl bytemuck::Pod for CameraUniform {}

pub struct Camera {
  position: Vector3<f32>, // 相机位置
  target: Vector3<f32>, // 相机目标
  up: Vector3<f32>, // 相机上方向
  fov: f32, // 视场角，横向广角
  screen_width: f32, // 屏幕宽度
  screen_height: f32, // 屏幕高度
  near: f32, // 近裁剪面
  far: f32, // 远裁剪面
}

impl Camera {
  pub fn new(position: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>, fov: f32, screen_width: f32, screen_height: f32, near: f32, far: f32) -> Self {
    Self {
      position,
      target,
      up,
      fov,
      screen_width,
      screen_height,
      near,
      far,
    }
  }

  
  pub fn model_matrix(&self) -> Matrix4<f32> {
    // 通过相机位置，将世界坐标通过模型矩阵转化为局部坐标
    Matrix4::new(
      2.0/self.screen_width, 0.0, 0.0, 0.0,
      0.0, -2.0/self.screen_height, 0.0, 0.0,
      0.0, 0.0, 0.001, 0.0,
      0.0, 0.0, 0.0, 1.0,
    )
  }

  // 通过相机获取视图矩阵
  pub fn view_matrix(&self) -> Matrix4<f32> {
    Matrix4::look_at_rh(&Point3::from(self.position), &Point3::from(self.target), &self.up)
    // 手动实现一下view矩阵的推导
    // 第一步，将相机移动到原点（0,0,0）
    // let t_view = Matrix4::new(
    //   1.0, 0.0, 0.0, -self.position.x,
    //   0.0, 1.0, 0.0, -self.position.y,
    //   0.0, 0.0, 1.0, -self.position.z,
    //   0.0, 0.0, 0.0, 1.0,
    // );
    // // 第二步，将相机的目标方向转换为相机的朝向
    // let z_axis = (self.target - self.position).normalize();
    // let x_axis = self.up.cross(&z_axis).normalize();
    // let y_axis = z_axis.cross(&x_axis);
    // // println!("Camera::view_matrix: {:?}, up: {:?}", &z_axis, &x_axis);
    // let t_rotate = Matrix4::new(
    //   x_axis.x, x_axis.y, x_axis.z, 0.0,
    //   y_axis.x, y_axis.y, y_axis.z, 0.0,
    //   z_axis.x, z_axis.y, z_axis.z, 0.0,
    //   0.0, 0.0, 0.0, 1.0,
    // );
    // // 第三步，将相机的朝向转换为世界坐标
    // let t_view = t_rotate * t_view;
    // t_view
    // t_rotate
  }

  // 通过相机获取投影矩阵
  pub fn projection_matrix(&self) -> Matrix4<f32> {
    // 手动实现透视投影的矩阵推导
    // 计算视锥体的宽度(因为坐标都是原点0为中心，左右对称的数据，因此这里只需要计算一半的宽度)
    // let width = (self.fov * 0.5).tan() * self.near;
    // println!("Camera::projection_matrix: width: {}", (self.fov * 0.5).tan());
    // // 计算视锥体的高度
    // let height = width / self.aspect;

    // // 计算视锥体的投影矩阵
    // let projection = Matrix4::new(
    //   1.0 / width, 0.0, 0.0, 0.0,
    //   0.0, 1.0 / height, 0.0, 0.0,
    //   0.0, 0.0, -self.far / (self.far - self.near), -1.0,
    //   0.0, 0.0, -(self.far * self.near) / (self.far - self.near), 0.0,
    // );
    // projection

    // let f = 1.0 / (self.fov * 0.5).tan();
    // let aspect = self.aspect;
    // let z_range = self.far - self.near;
    // let z_scale = 1.0 / (self.far - self.near);
    // let z_offset = -(self.far * self.near) / z_range;
    // let n = self.near;
    // let f = self.far;
    // let projection = Matrix4::new(
    //   n, 0.0, 0.0, 0.0,
    //   0.0, n, 0.0, 0.0,
    //   0.0, 0.0, n+f, -n*f,
    //   0.0, 0.0, 1.0, 0.0,
    // );
    // projection
    let aspect = self.screen_width / self.screen_height;
    Matrix4::new_perspective(aspect, self.fov, self.near, self.far)
    // Projective3::new()
  } 

  pub fn uniform_obj(&self) -> CameraUniform  {
    let camera_uniform = CameraUniform {
      proj: self.projection_matrix(),
      view: self.view_matrix(),
      model: self.model_matrix(),
      // proj: [
      //   [1.0, 0.0, 0.0, 0.0],
      //   [0.0, 1.0, 0.0, 0.0],
      //   [0.0, 0.0, 1.0, 0.0],
      //   [0.0, 0.0, 0.0, 1.0]].into(),
      // view: [
      //   [1.0, 0.0, 0.0, 0.0],
      //   [0.0, 1.0, 0.0, 0.0],
      //   [0.0, 0.0, 1.0, 0.0],
      //   [0.0, 0.0, 0.0, 1.0]].into(),
    };
    println!("Camera::uniform_obj: {:?}", &camera_uniform);
    camera_uniform
  }

  pub fn set_target(&mut self, target: Vector3<f32>) {
    self.target = target;
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
