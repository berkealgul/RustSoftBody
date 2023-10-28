use macroquad::prelude::*;
use crate::collisions::*;

#[derive(Debug, Clone)]
pub struct SoftMesh
{
    pub w: usize,
    pub h: usize,
    pub particle_mass: f32,
    pub K: f32,
    pub vertex_vec: Vec<Vertex>,
    pub edge_vec: Vec<Edge>
}

impl SoftMesh  {
    pub fn physics_step(&mut self, dt:f32, static_colliders:&Vec<StaticCollider>) {
        // step 1: calculate forces applied by springs(edge) and add them to vertices
        for edge in self.edge_vec.iter() {
            let v1 = self.vertex_vec[edge.v1_idx];
            let v2 = self.vertex_vec[edge.v2_idx];
            let I: Vec3 = v2.pos - v1.pos;
            let mut F: Vec3 = -edge.K * (I - edge.L * (I / I.length())); 

            self.vertex_vec[edge.v1_idx].add_force(-F); 
            self.vertex_vec[edge.v2_idx].add_force(F);
        }

        //let vertex_vec_copy = self.vertex_vec.clone();

        // step 2 update each vertex
        for vertex in self.vertex_vec.iter_mut() {
            if(vertex.is_static) { // skip static vertices
                continue;
            }

            // step 2.a update vertex v and a
            let g = vertex.m * Vec3{x: 0.0, y: 9.8, z:0.0}; // downwards is +y
            vertex.add_force(g);// add gravity to total force
            vertex.add_force(-vertex.c * vertex.v); // damping force
            vertex.a = vertex.f / vertex.m;
            vertex.v += vertex.a * dt;
            
            // NOT WORKING
            // step 2.b check for collisions with other vertices
            // for v2 in vertex_vec_copy.iter() {
            //     let dv = v2.pos - vertex.pos;
            //     let d = dv.length();
            //     if(d == 0.0) {
            //         continue;
            //     }

            //     if(d < 3.0 && dv.dot(vertex.v) > 0.0) {
            //        vertex.v -= vertex.v.length() * dv.normalize();
            //        break;
            //     } 
            // }

            // step 2.b check for collisions with static colliders and handle collision
            for collider in static_colliders {
                let c12 = collider.v2 - collider.v1;
                let cv = collider.v1 - vertex.pos;
                let d = (c12.x * cv.y - cv.x * c12.y).abs() / c12.length();
                
                let cv1 = collider.v1 - vertex.pos;
                let cv2 = collider.v2 - vertex.pos;

                // handle collision
                if d <= 6.0 && cv1.dot(cv2) < 0.0 {
                    let v = vertex.v.normalize();
                    let vx = vertex.v.length();
                    let w = (v -  2.0 * v.dot(collider.n) * collider.n);
                    vertex.v = w * vx;
                    break
                }
            }

            //step 2.c update pos 
            vertex.pos += vertex.v * dt; 
            vertex.f = Vec3::ZERO; // reset force for next update step
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
        self.w = w;
        self.h = h;

        for j in 0..h{
            for i in 0..w{
                let i_ = f32::from(i as i16 * offset) + initial_pos.x;
                let j_ = f32::from(j as i16 * offset) + initial_pos.y;
                let v: Vertex = Vertex::create_vertex(i_, j_, self.particle_mass);
                self.vertex_vec.push(v);

                if i > 0 {
                    self.edge_vec.push(Edge::create_edge((j * w) + i, (j * w) + i - 1, &self.vertex_vec, self.K))
                }
                if j > 0 {
                    self.edge_vec.push(Edge::create_edge((j * w) + i, ((j-1) * w) + i, &self.vertex_vec, self.K)) 
                }
                if i > 0 && j > 0 {   
                    self.edge_vec.push(Edge::create_edge((j * w) + i, ((j-1) * w) + i - 1, &self.vertex_vec, self.K))
                }
                if j > 0 && i < w - 1 {
                    self.edge_vec.push(Edge::create_edge((j * w) + i, ((j-1) * w) + i + 1, &self.vertex_vec, self.K))
                }
            }
        }   
    }

    pub fn change_static_state_of_vertex(&mut self, i: usize, j: usize, state:bool) 
    {
        self.vertex_vec[(j * self.w) + i].is_static = state;
    }

    pub fn create_mesh(w:usize, h: usize, initial_pos:Vec3, offset:i16, particle_mass:f32, K:f32) -> Self 
    {
        let mut mesh = SoftMesh{vertex_vec: vec!{}, 
                                        edge_vec: vec!{}, 
                                        w: 0, 
                                        h: 0, 
                                        particle_mass:particle_mass,
                                        K:K};

        mesh.init(w, h, initial_pos, offset);
        return mesh;
    }
    
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub v1_idx: usize,
    pub v2_idx: usize,
    pub L:  f32,
    pub K: f32,
}

impl Edge {
    pub fn create_edge(v1_idx:usize, v2_idx:usize, vertex_vec: &Vec<Vertex>, K:f32) -> Edge {
        let L = (vertex_vec[v2_idx].pos - vertex_vec[v1_idx].pos).length();
        Edge{v1_idx:v1_idx, v2_idx:v2_idx, L:L, K:K}
    }
}
 
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,  
    pub f: Vec3,
    pub m: f32,
    pub v: Vec3,
    pub a: Vec3, 
    pub c: f32,
    pub is_static: bool
}

impl Vertex {
    pub fn draw(&self) {
        draw_circle(f32::from(self.pos.x), f32::from(self.pos.y), 3.0, YELLOW);
    }

    pub fn create_vertex(x: f32, y: f32, m:f32) -> Self {
        Vertex {pos: Vec3 { x: x, y: y, z: 0.0 }, 
                f: Vec3::ZERO, 
                v: Vec3::ZERO,
                a: Vec3::ZERO,
                m: m,
                c: 1.0,
                is_static: false}
    }

    pub fn add_force(&mut self, f:Vec3) {
        self.f += f;
    }
}

#[derive(Debug, Clone)]
pub struct SoftBubble
{
    pub particle_mass: f32,
    pub K: f32,
    pub vertex_vec: Vec<Vertex>,
    pub edge_vec: Vec<Edge>
}

impl SoftBubble {
    
    pub fn init(&mut self, r:f32, initial_pos:Vec3, offset:i16)
    {
    
    }

    pub fn create_bubble() -> Self {
        SoftBubble { particle_mass: (), K: (), vertex_vec: (), edge_vec: () }
    }
    
}