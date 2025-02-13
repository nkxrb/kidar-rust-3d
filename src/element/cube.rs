use crate::render::vertex::Vertex;

pub struct Cube {
  pub w: f32,
  pub h: f32,
  pub d: f32,
  pub color: [f32; 3],
  pub pos: Vec<Vertex>,
}

impl Cube {
  pub fn new(w: f32, h: f32, d: f32, color: [f32; 3]) -> Self {
    let x = w/2.0;
    let y = h/2.0;
    let z = d/2.0;

    let pos = [
      // 前面
      Vertex { position: [x, y, z], color, tex_coords: [w, h] },
      Vertex { position: [x, -y, z], color, tex_coords: [w, h] },
      Vertex { position: [-x, -y, z], color, tex_coords: [w, h] },
      Vertex { position: [-x, -y, z], color, tex_coords: [w, h] },
      Vertex { position: [-x, y, z], color, tex_coords: [w, h] },
      Vertex { position: [x, y, z], color, tex_coords: [w, h] },

      // 后面
      Vertex { position: [x, y, -z], color, tex_coords: [w, h] },
      Vertex { position: [-x, y, -z], color, tex_coords: [w, h] },
      Vertex { position: [-x, -y, -z], color, tex_coords: [w, h] },
      Vertex { position: [-x, -y, -z], color, tex_coords: [w, h] },
      Vertex { position: [x, -y, -z], color, tex_coords: [w, h] },
      Vertex { position: [x, y, -z], color, tex_coords: [w, h] },

      // 上面
      Vertex { position: [x, y, -z], color, tex_coords: [w, h] },
      Vertex { position: [x, y, z], color, tex_coords: [w, h] },
      Vertex { position: [-x, y, z], color, tex_coords: [w, h] },
      Vertex { position: [-x, y, z], color, tex_coords: [w, h] },
      Vertex { position: [-x, y, -z], color, tex_coords: [w, h] },
      Vertex { position: [x, y, -z], color, tex_coords: [w, h] },

      // 下面
      Vertex { position: [x, -y, -z], color, tex_coords: [w, h] },
      Vertex { position: [-x, -y, -z], color, tex_coords: [w, h] },
      Vertex { position: [-x, -y, z], color, tex_coords: [w, h] },
      Vertex { position: [-x, -y, z], color, tex_coords: [w, h] },
      Vertex { position: [x, -y, z], color, tex_coords: [w, h] },
      Vertex { position: [x, -y, -z], color, tex_coords: [w, h] },
      // 左面
      Vertex { position: [-x, y, z], color, tex_coords: [w, h] },
      Vertex { position: [-x, y, -z], color, tex_coords: [w, h] },
      Vertex { position: [-x, -y, -z], color, tex_coords: [w, h] },
      Vertex { position: [-x, -y, z], color, tex_coords: [w, h] },
      Vertex { position: [-x, y, z], color, tex_coords: [w, h] },
      Vertex { position: [-x, y, -z], color, tex_coords: [w, h] },
      // 右面
      Vertex { position: [x, y, -z], color, tex_coords: [w, h] },
      Vertex { position: [x, y, z], color, tex_coords: [w, h] },
      Vertex { position: [x, -y, z], color, tex_coords: [w, h] },
      Vertex { position: [x, -y, z], color, tex_coords: [w, h] }, 
      Vertex { position: [x, -y, -z], color, tex_coords: [w, h] },
      Vertex { position: [x, y, -z], color, tex_coords: [w, h] },
      
    ].to_vec();

    Self {
      w,
      h,
      d,
      color,
      pos,
    }
  }
}