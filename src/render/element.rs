pub struct Element {
  pub x: i16, // 相对设备屏幕坐标而言，i16足够使用了
  pub y: i16,
  pub width: u16,
  pub height: u16,
  pub bg_color: [u8; 4],
}

impl Element {
  pub fn new(x: i16, y: i16, width: u16, height: u16, bg_color: [u8; 4]) -> Self {
    Self {
      x,
      y,
      width,
      height,
      bg_color,
    }
  }
}