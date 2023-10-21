use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct StaticCollider
{
    pub v1: Vec3,
    pub v2: Vec3,
    pub n: Vec3
}

impl StaticCollider
{
    pub fn init_collider(v1: Vec3, v2: Vec3) -> Self
    {
        // create normal
        let m = Vec3{
            x:(v2.x + v1.x) / 2.0,
            y:(v2.y + v1.y) / 2.0,
            z:0.0
        };

        let n_ = v1 - m;
        let n = Vec3{
                x: -n_.y,
                y:  n_.x,
                z: 0.0
            }.normalize();
            
        StaticCollider { v1: v1, v2: v2, n:n }
    }

    pub fn draw(&self) 
    {
        draw_line(self.v1.x, self.v1.y, self.v2.x, self.v2.y, 5.0, BLACK);

        // draw normal line
        // let m = Vec3{
        //                 x:(self.v2.x + self.v1.x) / 2.0,
        //                 y:(self.v2.y + self.v1.y) / 2.0,
        //                 z:0.0
        //             };

        // let mn = self.n * 50.0 + m;

        // draw_line(m.x, m.y, mn.x, mn.y, 3.0, BLUE);
    }

}