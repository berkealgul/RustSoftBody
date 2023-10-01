use macroquad::prelude::*;
//use mod math::

#[derive(Debug, Clone)]
pub struct SoftMesh
{
    pub vertex_vec: Vec<Vertex>,
    pub edge_vec: Vec<Edge>
}

impl SoftMesh  {
    pub fn physics_step(&mut self, dt:f32) {
        for edge in self.edge_vec.iter_mut() {
            edge.physics_step()
        }
        for vertex in self.vertex_vec.iter_mut() {
            vertex.physics_step(dt)
        }
    }

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
        for j in 0..h{
            for i in 0..w{
                let i_ = f32::from(i as i16 * 30);
                let j_ = f32::from(j as i16 * 30);
                let mut v: Vertex = Vertex::create_vertex(i_, j_);
                self.vertex_vec.push(v);
                
                let mut prev_vec: Vertex;

                if i > 0 {
                    prev_vec = self.vertex_vec[(j * w) + i - 1];  
                    self.edge_vec.push(Edge::create_edge(v, prev_vec))
                }
                if j > 0 {
                    prev_vec = self.vertex_vec[((j-1) * w) + i]; 
                    self.edge_vec.push(Edge::create_edge(v, prev_vec))  
                }
                if i > 0 && j > 0 {
                    prev_vec = self.vertex_vec[((j-1) * w) + i - 1];    
                    self.edge_vec.push(Edge::create_edge(v, prev_vec))
                }
                if j > 0 && i < w - 1 {
                    prev_vec = self.vertex_vec[((j-1) * w) + i + 1];    
                    self.edge_vec.push(Edge::create_edge(v, prev_vec))
                }
            }
        }   
    }
    
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub v1: Vertex,
    pub v2: Vertex,
    pub L:  f32,
    pub K: f32
}

impl Edge {
    pub fn draw(&self) {
        draw_line(self.v1.pos.x, self.v1.pos.y, self.v2.pos.x, self.v2.pos.y, 2.0, BLACK);
    }

    pub fn physics_step(&mut self) {
        let I: Vec3 = self.v2.pos - self.v1.pos;
        let F: Vec3 = -self.K * (I - self.L * (I / I.length())); 
        self.v1.f += F; 
        self.v2.f -= F;
    }

    pub fn create_edge(v1:Vertex, v2:Vertex) -> Edge {
        let L = (v1.pos - v2.pos).length();
        Edge{v1:v1, v2:v2, L:L, K:5.0}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub f: Vec3,
    pub m: f32,
    pub v: Vec3,
    pub a: Vec3, 
}

impl Vertex {
    pub fn draw(&self) {
        draw_circle(f32::from(self.pos.x), f32::from(self.pos.y), 5.0, YELLOW);
    }

    pub fn physics_step(&mut self, dt: f32) {
        let g = self.m * Vec3{x: 0.0, y: 9.8, z:0.0}; // downwards is +y
        self.f += g;// add gravity to total force
        self.a = self.f / self.m;
        self.v += self.a * dt;
        self.pos += self.v * dt; 
    }

    pub fn create_vertex(x: f32, y: f32) -> Self {
        Vertex {pos: Vec3 { x: x, y: y, z: 0.0 }, 
                f: Vec3::ZERO, 
                v: Vec3::ZERO,
                a: Vec3::ZERO,
                m: 1.0}
    }
}
