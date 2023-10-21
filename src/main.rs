pub mod math;
pub mod soft_structure;  
pub mod collisions;

use macroquad::prelude::*;
use soft_structure::*;
use collisions::*;

#[macroquad::main("SoftBodyPhysics")]
async fn main() {
    let mut mesh = SoftMesh::create_mesh(5, 8, 
        Vec3{x:50.0, y:20.0, z:0.0}, 
        30, 1.0, 15.0);

    // mesh.change_static_state_of_vertex(0, 0, true);
    // mesh.change_static_state_of_vertex(4, 0, true);

    let mut colliders:Vec<StaticCollider> = vec!{};
 
    colliders.push(StaticCollider::init_collider(
        Vec3{x:0.0, y:screen_height() - 2.0, z:0.0},
        Vec3{x:screen_width(), y:screen_height() - 2.0, z:0.0}));
    
    colliders.push(StaticCollider::init_collider(
            Vec3{x:0.0, y:300.0, z:0.0},
            Vec3{x:200.0, y:400.0, z:0.0}));

    colliders.push(StaticCollider::init_collider(
                Vec3{x:400.0, y:500.0, z:0.0},
                Vec3{x:500.0, y:300.0, z:0.0}));
    
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