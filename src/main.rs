pub mod math;
pub mod soft_structure;  

use macroquad::prelude::*;
use soft_structure::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut mesh = SoftMesh{vertex_vec: vec!{}, edge_vec: vec!{}};

    mesh.init(5, 5);

    loop {
        clear_background(RED);

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);


        mesh.draw();
        mesh.physics_step(0.001);


        next_frame().await
        
    }
}