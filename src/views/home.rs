use crate::{element::{action::move_el, cube::Cube}, render::vertex::Vertex};


// struct Home

pub fn draw_home() -> Vec<Vertex>{
  // 绘制多个立方体
  let init_objs = [
    Cube::new(5000.0, 2000.0, 2500.0, 200.0, 200.0,200.0,0.5),
    Cube::new(5200.0, 2200.0, 2200.0, 100.0, 100.0, 100.0, 0.2),
    Cube::new(5100.0, 2200.0, 2325.0, 50.0, 50.0, 50.0, 0.9),
    Cube::new(5200.0, 2200.0, 2325.0, 50.0, 50.0, 50.0, 0.9),
    Cube::new(5300.0, 2200.0, 2325.0, 50.0, 50.0, 50.0, 0.9),
    Cube::new(5400.0, 2200.0, 2325.0, 50.0, 50.0, 50.0, 0.9),
    Cube::new(5500.0, 2200.0, 2325.0, 50.0, 50.0, 50.0, 0.9),
    Cube::new(4800.0, 2200.0, 2325.0, 50.0, 50.0, 50.0, 0.9),
  ];
  
  // 将两个立方体的顶点合并
  let mut vertex_list = vec![];
  for obj in init_objs.iter() {
    vertex_list.extend_from_slice(&obj.pos);
  }
  // let all_pos = cube_a.pos.iter().chain(&cube_b.pos).chain(&cube_c.pos).cloned().collect();
  
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
  vertex_list
}