use crate::render::vertex::Vertex;

pub struct Cube {
  pub w: f32,
  pub h: f32,
  pub d: f32,
  pub cx: f32,
  pub cy: f32,
  pub cz: f32,
  pub pos: Vec<Vertex>,
  pub c: f32,
}

impl Cube {
  pub fn new(cx: f32, cy: f32, cz: f32, w: f32, h: f32, d: f32, c: f32) -> Self {
    let x = cx + w/2.0;
    let y = cy + h/2.0;
    let z = cz + d/2.0;
    let x2 = cx - w/2.0;
    let y2 = cy - h/2.0;
    let z2 = cz - d/2.0;    

    let pos = [
      // 前面
      Vertex { position: [x, y, z], color: [0.0, c, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y, z], color: [0.0, c, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y2, z], color: [0.0, c, 0.0], tex_coords: [w, h] },
      Vertex { position: [x, y, z], color: [0.0, c, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y2, z], color: [0.0, c, 0.0], tex_coords: [w, h] },
      Vertex { position: [x, y2, z], color: [0.0, c, 0.0], tex_coords: [w, h] },

      // 后面
      Vertex { position: [x, y, z2], color: [c, 0.0, 0.0], tex_coords: [w, h] },
      Vertex { position: [x, y2, z2], color: [c, 0.0, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y2, z2], color: [c, 0.0, 0.0], tex_coords: [w, h] },
      Vertex { position: [x, y, z2], color: [c, 0.0, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y2, z2], color: [c, 0.0, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y, z2], color: [c, 0.0, 0.0], tex_coords: [w, h] },

      // 上面
      Vertex { position: [x, y, z2], color: [0.0, 0.0, c], tex_coords: [w, h] },
      Vertex { position: [x2, y, z2], color: [0.0, 0.0, c], tex_coords: [w, h] },
      Vertex { position: [x2, y, z], color: [0.0, 0.0, c], tex_coords: [w, h] },
      Vertex { position: [x, y, z2], color: [0.0, 0.0, c], tex_coords: [w, h] },
      Vertex { position: [x2, y, z], color: [0.0, 0.0, c], tex_coords: [w, h] },
      Vertex { position: [x, y, z], color: [0.0, 0.0, c], tex_coords: [w, h] },

      // 下面
      Vertex { position: [x, y2, z2], color: [0.0, c, 0.5], tex_coords: [w, h] },
      Vertex { position: [x, y2, z], color: [0.0, c, 0.5], tex_coords: [w, h] },
      Vertex { position: [x2, y2, z], color: [0.0, c, 0.5], tex_coords: [w, h] },
      Vertex { position: [x, y2, z2], color: [0.0, c, 0.5], tex_coords: [w, h] },
      Vertex { position: [x2, y2, z], color: [0.0, c, 0.5], tex_coords: [w, h] },
      Vertex { position: [x2, y2, z2], color: [0.0, c, 0.5], tex_coords: [w, h] },
      // 左面
      Vertex { position: [x2, y, z], color: [1.5, c, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y, z2], color: [1.5, c, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y2, z2], color: [1.5, c, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y, z], color: [0.5, c, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y2, z2], color: [0.5, c, 0.0], tex_coords: [w, h] },
      Vertex { position: [x2, y2, z], color: [0.5, c, 0.0], tex_coords: [w, h] },
      
      // 右面
      Vertex { position: [x, y, z2], color: [0.0, c, 0.2], tex_coords: [w, h] },
      Vertex { position: [x, y, z], color: [0.0, c, 0.2], tex_coords: [w, h] },
      Vertex { position: [x, y2, z], color: [0.0, c, 0.2], tex_coords: [w, h] },
      Vertex { position: [x, y, z2], color: [0.0, c, 0.2], tex_coords: [w, h] },
      Vertex { position: [x, y2, z], color: [0.0, c, 0.2], tex_coords: [w, h] }, 
      Vertex { position: [x, y2, z2], color: [0.0, c, 0.2], tex_coords: [w, h] },
      
    ].to_vec();

    Self {
      cx,
      cy,
      cz,
      w,
      h,
      d,
      c,
      pos,
    }
  }
}