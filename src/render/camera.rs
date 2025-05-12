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

  // 将世界坐标转为相机坐标的变化矩阵
  pub fn world_to_camera(&self) -> Matrix4<f32> {
    let camera_pos = self.position;
    // 先转化为基于相机位置的局部屏幕坐标
    let relate_pos = Matrix4::new(
      1.0, 0.0, 0.0, -camera_pos.x,
      0.0, 1.0, 0.0, -camera_pos.y,
      0.0, 0.0, 1.0, -camera_pos.z,
      0.0, 0.0, 0.0, 1.0,
    );

    let camera_target = self.target;
    let camera_up = self.up;
    let z_axis = (camera_target - camera_pos).normalize();
    let x_axis = camera_up.cross(&z_axis).normalize();
    let y_axis = z_axis.cross(&x_axis);
    let camera_matrix = Matrix4::new(
      x_axis.x, x_axis.y, x_axis.z, 0.0,
      y_axis.x, y_axis.y, y_axis.z, 0.0,
      z_axis.x, z_axis.y, z_axis.z, 0.0,
      0.0, 0.0, 0.0, 1.0,
    );

    relate_pos
  }

  
  pub fn model_matrix(&self) -> Matrix4<f32> {
    // 通过相机位置，将世界坐标通过模型矩阵转化为局部坐标
    let w = 2.0/self.screen_width;
    let h = 2.0/self.screen_height;
    let d = 0.001;
    Matrix4::new(
      w, 0.0, 0.0, -self.position.x*w,
      0.0, h, 0.0, -self.position.y*h,
      0.0, 0.0, d, -self.position.z*d,
      0.0, 0.0, 0.0, 1.0,
    )

    // return  Matrix4::new(
    //   w, 0.0, 0.0, 0.0,
    //   0.0, h, 0.0, 0.0,
    //   0.0, 0.0, d, 0.0,
    //   0.0, 0.0, 0.0, 1.0,
    // );
  }

  // 通过相机获取视图矩阵
  pub fn view_matrix(&self) -> Matrix4<f32> {
    // Matrix4::look_at_rh(&Point3::from(self.position), &Point3::from(self.target), &self.up)
    // 手动实现一下view矩阵的推导
    // 第一步，将相机移动到原点（0,0,0）
    // let t_view = Matrix4::new(
    //   1.0, 0.0, 0.0, -self.position.x,
    //   0.0, 1.0, 0.0, -self.position.y,
    //   0.0, 0.0, 1.0, -self.position.z,
    //   0.0, 0.0, 0.0, 1.0,
    // );
    // 第二步，将相机的目标方向转换为相机的朝向
    let z_axis = (self.target - self.position).normalize();
    let x_axis = self.up.cross(&z_axis).normalize();
    let y_axis = z_axis.cross(&x_axis);
    // println!("Camera::view_matrix: {:?}, up: {:?}", &z_axis, &x_axis);
    let t_rotate = Matrix4::new(
      x_axis.x, x_axis.y, x_axis.z, 0.0,
      y_axis.x, y_axis.y, y_axis.z, 0.0,
      -z_axis.x, -z_axis.y, -z_axis.z, 0.0,
      0.0, 0.0, 0.0, 1.0,
    );
    // 第三步，将相机的朝向转换为世界坐标
    // t_rotate * t_view
    t_rotate
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
    let aspect = self.screen_height / self.screen_width;
    Matrix4::new_perspective(1.0, self.fov, self.near, self.far)
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
    // println!("Camera::uniform_obj: {:?}", &camera_uniform);
    // 矩阵点乘
    camera_uniform
  }

  pub fn set_screen_size(&mut self, screen_width: f32, screen_height: f32) {
    self.screen_width = screen_width;
    self.screen_height = screen_height;
  }

  pub fn set_target(&mut self, target: Vector3<f32>) {
    self.target = target;
  }
  pub fn set_position(&mut self, position: Vector3<f32>) {
    self.position = position;
  }

  // 向前移动
  pub fn move_forward(&mut self, distance: f32) {
    let forward = (self.target - self.position).normalize();
    self.position += forward * distance;
    self.target += forward * distance;
  }
  // 向后移动
  pub fn move_backward(&mut self, distance: f32) {
    let backward = (self.position - self.target).normalize();
    self.position += backward * distance;
    self.target += backward * distance;
  }
  // 向左移动
  pub fn move_left(&mut self, distance: f32) {
    let left = self.up.cross(&(self.target - self.position)).normalize();
    self.position += left * distance;
    self.target += left * distance;
  }
  // 向右移动
  pub fn move_right(&mut self, distance: f32) {
    let right = (self.target - self.position).cross(&self.up).normalize();
    self.position += right * distance;
    self.target += right * distance;
  }

  // 抬头
  pub fn look_up(&mut self, angle: f32) {
    let z_axis = (self.target - self.position).normalize();
    let x_axis = self.up.cross(&z_axis).normalize();
    // 使用四元数旋转
    let rotation = UnitQuaternion::from_axis_angle(&Unit::new_normalize(x_axis), angle.to_radians());
    let new_z_axis = rotation * (self.target - self.position);
    self.target = self.position + new_z_axis;
    println!("rotation: {:?}, tar: {:?}", &rotation, &self.target);
    self.up = (self.target - self.position).normalize().cross(&x_axis);
  }
  // 低头
  pub fn look_down(&mut self, angle: f32) {
    self.look_up(-angle);
  }

  // 向上移动
  pub fn move_up(&mut self, distance: f32) {
    self.position += self.up * distance;
  }

  // 向下移动
  pub fn move_down(&mut self, distance: f32) {
    self.position -= self.up * distance;
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
