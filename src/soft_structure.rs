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

    pub fn init(&mut self, w:usize, h: usize)
    {
        let mut vertex_grid: Vec<Vec<Option<*mut Vertex>>> = vec![vec![None; w]; h];

        for i in 1..w{
            for j in 1..h{
                let i_ = f32::from(i as i16 * 10);
                let j_ = f32::from(j as i16 * 10);
                let mut v: Vertex = Vertex { pos: Vec3 { x: i_, y: j_, z: 0.0 } };
                self.vertex_vec.push(v);
                vertex_grid[i][j] = Some(&mut v);
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
