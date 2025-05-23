use std::f32::consts::{FRAC_PI_2, PI};

use nalgebra::{Matrix4, Point3, Unit, UnitQuaternion, Vector3};
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

pub enum CameraMove {
  Forward,
  Backward,
  Left,
  Right,
  Up,
  Down,
  None,  
}

pub struct Camera {
  active_status: bool, // 相机是否激活
  position: Vector3<f32>, // 相机位置
  target: Vector3<f32>, // 相机目标
  up: Vector3<f32>, // 相机上方向
  fov: f32, // 视场角，横向广角
  screen_width: f32, // 屏幕宽度
  screen_height: f32, // 屏幕高度
  near: f32, // 近裁剪面
  far: f32, // 远裁剪面
  sensitivity: f32, // 鼠标灵敏度
  forward: Vector3<f32>,
  yaw: f32, // 偏航角
  pitch: f32, // 俯仰角
  speed: f32, // 相机移动速度
  pub is_forward: bool, // 是否向前移动
  pub is_backward: bool,
  pub is_left: bool,
  pub is_right: bool,
  pub is_up: bool,
  pub is_down: bool,
}

impl Camera {
  pub fn new(position: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>, fov: f32, screen_width: f32, screen_height: f32, near: f32, far: f32, sensitivity: f32) -> Self {
    Self {
      active_status: false,
      position,
      target,
      up,
      fov,
      screen_width,
      screen_height,
      near,
      far,
      sensitivity,
      forward: (target - position).normalize(),
      yaw: 0.0,
      pitch: 0.0,
      speed: 5.0,
      is_forward: false,
      is_backward: false,
      is_left: false,
      is_right: false,
      is_up: false,
      is_down: false,
    }
  }
  
  pub fn model_matrix(&self) -> Matrix4<f32> {
    // 通过相机位置，将世界坐标通过模型矩阵转化为局部坐标
    let w = 2.0/self.screen_width;
    let h = 2.0/self.screen_height;
    let d = 0.001; // 场景最大裁剪厚度是1000
    Matrix4::new(
      w, 0.0, 0.0, -self.position.x*w,
      0.0, h, 0.0, -self.position.y*h,
      0.0, 0.0, d, -self.position.z*d,
      0.0, 0.0, 0.0, 1.0,
    )
  }

  // 通过相机获取视图矩阵
  pub fn view_matrix(&self) -> Matrix4<f32> {
    let z_axis = self.target;
    let x_axis = self.up.cross(&z_axis).normalize();
    let y_axis = z_axis.cross(&x_axis);
    let t_rotate = Matrix4::new(
      x_axis.x, x_axis.y, x_axis.z, 0.0,
      y_axis.x, y_axis.y, y_axis.z, 0.0,
      -z_axis.x, -z_axis.y, -z_axis.z, 0.0,
      0.0, 0.0, 0.0, 1.0,
    );
    t_rotate
  }

  // 通过相机获取投影矩阵
  pub fn projection_matrix(&self) -> Matrix4<f32> {
    // aspect固定为1.0，方便实现正方形图形渲染
    // let aspect = self.screen_height / self.screen_width;
    Matrix4::new_perspective(1.0, self.fov, self.near, self.far)
  } 

  pub fn uniform_obj(&self) -> CameraUniform  {
    let proj = self.projection_matrix();
    let view = self.view_matrix();
    let model = self.model_matrix();
    let pvm = proj * view * model;
    // let pvm = Matrix4::new(
    //   -0.0021650565, 0.0, 3.2413034e-6, 3.2348273e-6, 
    //   0.0, 0.003849002, 0.0, 0.0, 
    //   -4.4823087e-6,0.0, -0.0010019987, -0.0009999967, 
    //   11.041788, -8.467804, -0.21673085, -0.01649762
    // ).transpose();
    // let pvm = Matrix4::new(
    //   0.0021650416, 0.0, -5.6631075e-6, -5.6517924e-6, 0.0, 0.003849002, 0.0, 0.0, 7.831355e-6, 0.0, 0.0010019918, 0.0009999898, -11.041712, -8.467804, -0.17131835, 0.028824143
    // ).transpose();

    let camera_uniform = CameraUniform {
      proj: pvm,
      view: view,
      model: model,
    };

    // println!("Camera::uniform_obj: {:?}", &camera_uniform);
    // println!("Camera::pvm: {:?}", &pvm);
    // println!("Camera::mvp: {:?}", &mvp);
    // 矩阵点乘
    camera_uniform

  }

  pub fn set_screen_size(&mut self, screen_width: f32, screen_height: f32) {
    self.screen_width = screen_width;
    self.screen_height = screen_height;
  }

  pub fn active_move(&mut self, status: bool) {
    self.active_status = status;
  }

  pub fn update(&mut self, dt: f32) {
    if !self.active_status { // 如果相机未激活，则不进行移动
      return
    }

    let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
    // z轴正向为正前方
    let forward = Vector3::new(
      yaw_sin,
      0.0,
      yaw_cos
    ).normalize();
    let right = self.up.cross(&forward).normalize();

    let forward_amount = if self.is_forward {1.0f32} else {0.0f32};
    let backward_amount = if self.is_backward {1.0f32} else {0.0f32};
    let left_amount = if self.is_left {1.0f32} else {0.0f32};
    let right_amount = if self.is_right {1.0f32} else {0.0f32};
    let up_amount = if self.is_up {1.0f32} else {0.0f32};
    let down_amount = if self.is_down {1.0f32} else {0.0f32};

    self.position += forward * (forward_amount - backward_amount) * self.speed;
    self.position += right * (right_amount - left_amount) * self.speed;
    self.position += self.up * (up_amount - down_amount) * self.speed;
    // println!("Camera::update: {:?}， {:?}", &self.position, &self.yaw);
    let (pitch_sin, pitch_cos) = self.pitch.sin_cos();

    self.target = Vector3::new(
      pitch_cos * yaw_sin,
      pitch_sin, 
      pitch_cos * yaw_cos, 
    ).normalize();
    // println!("Camera::update: {:?}， {:?}", &self.position, &self.target);
  }

  // 摄像头旋转
  pub fn look_rotate(&mut self, mouse_pos: (f64, f64), dt: f32) {
    if !self.active_status { // 如果相机未激活，则不进行旋转
      return
    }
    self.yaw += (mouse_pos.0 as f32) * self.sensitivity * dt;
    self.pitch += (mouse_pos.1 as f32) * self.sensitivity * dt;
    self.pitch = self.pitch.clamp(-FRAC_PI_2 + 0.1, FRAC_PI_2 - 0.1);
    // println!("Camera::look_rotate: {:?}, {:?}, {:?}", mouse_pos, &self.yaw, &self.pitch);
    // println!("Camera::look_sincos: {:?}, {:?}, {:?}", mouse_pos, &self.yaw.sin_cos(), &self.pitch.sin_cos());
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
