use super::element::Element;

use super::vertex::Vertex;

pub struct Layout {
  pub width: u16,
  pub height: u16,
  pub depth: u8,
  pub max_width: u16,
  pub max_height: u16,
  pub min_width: u16,
  pub min_height: u16,
  pub elements: Vec<Element>,
  pos_tl: (i16, i16),
  pos_tr: (i16, i16),
  pos_br: (i16, i16),
  pos_bl: (i16, i16),
}

impl Layout {
  
  pub fn new(width: u16, height: u16, depth: u8) -> Self {
    Layout {
      width,
      height,
      depth,
      max_width: width,
      max_height: height,
      min_width: width,
      min_height: height,
      elements: Vec::new(),
      pos_tl: (0, 0),
      pos_tr: (0, 0),
      pos_br: (0, 0),
      pos_bl: (0, 0),
    }
  }

  // 流式布局，从左到右，从上到下， 超出边界自动换行
  pub fn add_element(&mut self, width: u16, height: u16) {
    println!("add element x: {:#?}, y: {:#?}", self.pos_tl.0, self.pos_tl.1);
    let element = Element::new(self.pos_tl.0, self.pos_tl.1, width, height, [0, 0, 0, 100]);
    self.elements.push(element);
    self.pos_tl.0 = self.pos_tl.0 + width as i16;
    self.pos_tl.1 = self.pos_tl.1 + height as i16;
  }

  pub fn get_vertexs(&self) -> Vec<Vertex> {
    // TODO: Implement
    return vec![];
  }



}