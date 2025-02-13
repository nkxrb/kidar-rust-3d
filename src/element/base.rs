pub struct BaseElement {
  pub pos: Vec<Vertex>,
}

impl BaseElement {
  pub fn new() -> Self {
    Self {
      pos: vec![],
    }
  }
}