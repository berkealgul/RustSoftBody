pub mod math;
pub mod soft_structure;  
pub mod collisions;

use macroquad::prelude::*;
use soft_structure::*;
use collisions::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut mesh = SoftMesh{vertex_vec: vec!{}, edge_vec: vec!{}};

    mesh.init(5, 10, Vec3{x:50.0, y:20.0, z:0.0}, 20);

    let mut colliders:Vec<StaticCollider> = vec!{};
 
    colliders.push(StaticCollider::init_collider(
        Vec3{x:0.0, y:screen_height() - 5.0, z:0.0},
        Vec3{x:screen_width(), y:screen_height() - 5.0, z:0.0}));
    
    colliders.push(StaticCollider::init_collider(
            Vec3{x:0.0, y:300.0, z:0.0},
            Vec3{x:300.0, y:400.0, z:0.0}));

    loop {
        clear_background(GRAY);
        
        mesh.draw();
        for c in colliders.iter() {
            c.draw()
        }
        mesh.physics_step(0.05, &colliders);

        next_frame().await        
    }
}