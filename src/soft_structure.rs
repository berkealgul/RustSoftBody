use macroquad::prelude::*;


#[derive(Debug, Clone)]
pub struct SoftMesh
{
    pub vertex_vec: Vec<Vertex>,
    pub edge_vec: Vec<Edge>
}

impl SoftMesh  {
    pub fn draw(&self) {
        for vertex in self.vertex_vec.iter() {
            vertex.draw()
        }
    }    

    pub fn init(&mut self)
    {
        for i in 1..10 as i16{
            for j in 1..10 as i16{
                let i = f32::from(i * 10);
                let j = f32::from(j * 10);
                self.vertex_vec.push(Vertex { pos: Vec3 { x: i, y: j, z: 0.0 } })
            }
        }
    }
    
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {

}


#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3
}

impl Vertex {
    pub fn draw(&self) {
        draw_circle(f32::from(self.pos.x), f32::from(self.pos.y), 15.0, YELLOW);
    }
}
