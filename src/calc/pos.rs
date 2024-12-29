

pub fn screenPosToWorldPos(window_size: PhysicalSize<u32>, mouse_pos: PhysicalPosition<u32>, camera: &Camera) -> Vec3 {
  let x = (mouse_pos.x as f32 / window_size.width as f32) * 2.0 - 1.0;
  let y = (mouse_pos.y as f32 / window_size.height as f32) * 2.0 - 1.0;
  let z = -1.0;

  let ray = camera.get_ray(x, y);
  let mut pos = ray.at(100.0);
  pos.y = -pos.y;
  pos.z = -pos.z;
}