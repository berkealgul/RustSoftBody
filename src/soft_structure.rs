use macroquad::prelude::*;


#[derive(Debug, Clone)]
pub struct SoftMesh
{
    pub vertex_vec: Vec<Vertex>,
    pub edge_vec: Vec<Edge>
}

impl SoftMesh  {
    pub fn draw(&self) {
        for edge in self.edge_vec.iter() {
            edge.draw()
        }
        for vertex in self.vertex_vec.iter() {
            vertex.draw()
        }
    }
  
    pub fn init(&mut self, w:usize, h: usize)
    {
        //let mut vertex_grid: Vec<Vec<Option<*mut Vertex>>> = vec![vec![None; w]; h];

        for j in 0..h{
            for i in 0..w{
                let i_ = f32::from(i as i16 * 30);
                let j_ = f32::from(j as i16 * 30);
                let mut v: Vertex = Vertex { pos: Vec3 { x: i_, y: j_, z: 0.0 } };
                self.vertex_vec.push(v);
                
                let mut prev_vec: Vertex;

                if i > 0 {
                    prev_vec = self.vertex_vec[(j * w) + i - 1];  
                    self.edge_vec.push(Edge{v1:v, v2: prev_vec})
                }
                if j > 0 {
                    prev_vec = self.vertex_vec[((j-1) * w) + i]; 
                    self.edge_vec.push(Edge{v1:v, v2: prev_vec})  
                }
                if i > 0 && j > 0 {
                    prev_vec = self.vertex_vec[((j-1) * w) + i - 1];    
                    self.edge_vec.push(Edge{v1:v, v2: prev_vec})
                }
                if j > 0 && i < w - 1 {
                    prev_vec = self.vertex_vec[((j-1) * w) + i + 1];    
                    self.edge_vec.push(Edge{v1:v, v2: prev_vec})
                }
            }
        }   
    }
    
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub v1: Vertex,
    pub v2: Vertex
}

impl Edge {
    pub fn draw(&self) {
        draw_line(self.v1.pos.x, self.v1.pos.y, self.v2.pos.x, self.v2.pos.y, 2.0, BLACK);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3
}

impl Vertex {
    pub fn draw(&self) {
        draw_circle(f32::from(self.pos.x), f32::from(self.pos.y), 5.0, YELLOW);
    }
}
