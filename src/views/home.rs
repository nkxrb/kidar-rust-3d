use crate::{element::{action::move_el, cube::Cube}, render::vertex::Vertex};


// struct Home

pub fn draw_home() -> Vec<Vertex>{
  // 绘制一个操作面板
  let mut cube_a = Cube::new(5000.0, 2000.0, 500.0, 200.0, 200.0,200.0,0.5);
  let mut cube_b = Cube::new(5200.0, 1850.0, 200.0, 100.0, 100.0, 100.0, 0.2);
  let mut cube_c = Cube::new(5100.0, 2200.0, 325.0, 50.0, 50.0, 50.0, 0.9);
  
  // 将两个立方体的顶点合并
  // let mut vertex_list = vec![];
  // let merged_pos = cube_a.pos.clone();
  // merged_pos.extend(cube_b.pos.clone());
  let all_pos = cube_a.pos.iter().chain(&cube_b.pos).chain(&cube_c.pos).cloned().collect();
  
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