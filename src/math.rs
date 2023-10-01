use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 { 
    pub x: f32,
    pub y: f32,
    pub z: f32  
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul for Vec3 {
    type Output = f32;
    
    // Dot Product
    fn mul(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vec3 {
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn abs(&self) -> Self {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs()
        }
    }
}

// INTEGRATIONS