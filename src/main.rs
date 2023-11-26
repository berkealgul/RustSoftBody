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
        30, 1.0, 14.0);

    let mut mesh2 = SoftMesh::create_mesh(7, 10, 
            Vec3{x:600.0, y:20.0, z:0.0}, 
            30, 1.0, 12.0);
    
    let dt: f32 = 0.05;

    // pin mesh2 on upper corner
    mesh2.change_static_state_of_vertex(0, 0, true);
    mesh2.change_static_state_of_vertex(6, 0, true);

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
        mesh2.draw();
        for c in colliders.iter() {
            c.draw()
        }
        mesh.physics_step(dt, &colliders);
        mesh2.physics_step(dt, &colliders);

        next_frame().await        
    }
}