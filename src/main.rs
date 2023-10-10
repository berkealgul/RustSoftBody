pub mod math;
pub mod soft_structure;  

use macroquad::prelude::*;
use soft_structure::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut mesh = SoftMesh{vertex_vec: vec!{}, edge_vec: vec!{}};

    mesh.init(5, 5, Vec3{x:50.0, y:20.0, z:0.0}, 100);

    loop {
        clear_background(RED);

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        
        let mouse_pos = mouse_position();
        let x = mouse_pos.0;
        let y = mouse_pos.1;
        
        //mesh.vertex_vec[0].pos = Vec3{x:x, y:y, z:0.0};

        mesh.draw();
        mesh.physics_step(0.01);
        

        next_frame().await        
    }
}