use crate::render::vertex::Vertex;

pub fn move_el(vertexs: &mut Vec<Vertex>, dx: f32, dy: f32, dz: f32) {
  for i in 0..vertexs.len() {
    vertexs[i].position[0] += dx;
    vertexs[i].position[1] += dy;
    vertexs[i].position[2] += dz;
  }
}