use crate::{element::{action::move_el, cube::Cube}, render::vertex::Vertex};

pub fn draw_home() -> Vec<Vertex>{
  // 绘制一个操作面板
  let mut cube_a = Cube::new(200.0, 200.0, 200.0, [0.5, 0.0, 0.0]);
  let mut cube_b = Cube::new(100.0, 100.0, 100.0, [0.0, 0.5, 0.0]);
  move_el(&mut cube_a.pos, 200.0, 300.0, 0.0);
  println!("{:?}", &(&cube_a).pos);
  // 将两个立方体的顶点合并
  // let mut vertex_list = vec![];
  // let merged_pos = cube_a.pos.clone();
  // merged_pos.extend(cube_b.pos.clone());
  let all_pos = cube_a.pos.iter().chain(&cube_b.pos).cloned().collect();
  
  // vertex_list.extend(&(&cube_a).pos.to_vec());
  // vertex_list.extend(&cube_b.pos);
  // cube_a.pos.
  // for i in 0..cube_a.pos.len() {
  //   vertex_list.push(cube_a.pos[i]);
  // }
  // for i in 0..cube_b.pos.len() {
  //   vertex_list.push(cube_b.pos[i]);
  // }
  // vertex_list
  all_pos
}