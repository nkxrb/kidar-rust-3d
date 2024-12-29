struct Camera {
  position: Vector3<f32>, // 相机位置
  target: Vector3<f32>, // 相机目标
  up: Vector3<f32>, // 相机上方向
  fov: f32, // 视场角
  aspect: f32, // 宽高比
  near: f32, // 近裁剪面
  far: f32, // 远裁剪面
}

pub impl Camera {
  fn new(position: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>, fov: f32, aspect: f32, near: f32, far: f32) -> Self {
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

  fn view_matrix(&self) -> Matrix4<f32> {
    Matrix4::look_at_rh(self.position, self.target, self.up)
  }

  fn projection_matrix(&self) -> Matrix4<f32> {
    Matrix4::perspective_rh(self.fov, self.aspect, self.near, self.far)
  }
}