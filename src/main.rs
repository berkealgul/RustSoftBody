pub mod utils;

use utils::Vec3;

fn main() {
    let p1: Vec3<f64> = Vec3 { x: 1.2, y: 3.4, z: 0.0 };
    let p2: Vec3<f64> = Vec3 { x: 5.6, y: 7.8, z: 0.0 };

    let result: Vec3<f64> = p1 + p2;

    println!("{:?}", result); // Outputs: Point { x: 6.8, y: 11.2 }
}