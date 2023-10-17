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
        for edge in self.edge_vec.iter() {
            let v1 = self.vertex_vec[edge.v1_idx];
            let v2 = self.vertex_vec[edge.v2_idx];
            let I: Vec3 = v2.pos - v1.pos;
            let F: Vec3 = -edge.K * (I - edge.L * (I / I.length())); 
            self.vertex_vec[edge.v1_idx].add_force(-F); 
            self.vertex_vec[edge.v2_idx].add_force(F);
        }

        for vertex in self.vertex_vec.iter_mut() {
            if vertex.pos.y > 400.0 {
                vertex.add_force(Vec3 { x: (0.0), y: (-100.0), z: (0.0) })
            }

            let g = vertex.m * Vec3{x: 0.0, y: 9.8, z:0.0}; // downwards is +y
            vertex.add_force(g);// add gravity to total force
            vertex.add_force(-vertex.c * vertex.v); // damping force
            vertex.a = vertex.f / vertex.m;
            vertex.v += vertex.a * dt;
            vertex.pos += vertex.v * dt; 
            vertex.f = Vec3::ZERO;
        }
    }

    pub fn draw(&self) {
        for edge in self.edge_vec.iter() {
            let v1 = self.vertex_vec[edge.v1_idx];
            let v2 = self.vertex_vec[edge.v2_idx];
            draw_line(v1.pos.x, v1.pos.y, v2.pos.x, v2.pos.y, 2.0, BLACK);
        }
        for vertex in self.vertex_vec.iter() {
            vertex.draw()
        }
    }
  
    pub fn init(&mut self, w:usize, h: usize, initial_pos:Vec3, offset:i16)
    {
        for j in 0..h{
            for i in 0..w{
                let i_ = f32::from(i as i16 * offset) + initial_pos.x;
                let j_ = f32::from(j as i16 * offset) + initial_pos.y;
                let v: Vertex = Vertex::create_vertex(i_, j_);
                self.vertex_vec.push(v);

                if i > 0 {
                    self.edge_vec.push(Edge::create_edge((j * w) + i, (j * w) + i - 1, &self.vertex_vec))
                }
                if j > 0 {
                    self.edge_vec.push(Edge::create_edge((j * w) + i, ((j-1) * w) + i, &self.vertex_vec)) 
                }
                if i > 0 && j > 0 {   
                    self.edge_vec.push(Edge::create_edge((j * w) + i, ((j-1) * w) + i - 1, &self.vertex_vec))
                }
                if j > 0 && i < w - 1 {
                    self.edge_vec.push(Edge::create_edge((j * w) + i, ((j-1) * w) + i + 1, &self.vertex_vec))
                }
            }
        }   
    }
    
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub v1_idx: usize,
    pub v2_idx: usize,
    pub L:  f32,
    pub K: f32
}

impl Edge {
    pub fn create_edge(v1_idx:usize, v2_idx:usize, vertex_vec: &Vec<Vertex>) -> Edge {
        let L = (vertex_vec[v2_idx].pos - vertex_vec[v1_idx].pos).length();
        Edge{v1_idx:v1_idx, v2_idx:v2_idx, L:L, K:5.0}
    }
}
 
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub f: Vec3,
    pub m: f32,
    pub v: Vec3,
    pub a: Vec3, 
    pub c: f32
}

impl Vertex {
    pub fn draw(&self) {
        draw_circle(f32::from(self.pos.x), f32::from(self.pos.y), 5.0, YELLOW);
    }

    pub fn create_vertex(x: f32, y: f32) -> Self {
        Vertex {pos: Vec3 { x: x, y: y, z: 0.0 }, 
                f: Vec3::ZERO, 
                v: Vec3::ZERO,
                a: Vec3::ZERO,
                m: 1.0,
                c: 0.5}
    }

    pub fn add_force(&mut self, f:Vec3) {
        self.f += f;
    }
}
