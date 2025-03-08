use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct PVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PVec3 {
    pub const ZERO: Self = Self {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn rotate_x(&self, angle: f32) -> Self {
        Self {
            x: self.x,
            y: self.y * angle.cos() - self.z * angle.sin(),
            z: self.y * angle.sin() + self.z * angle.cos(),
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Self {
        Self {
            x: self.x * angle.cos() - self.z * angle.sin(),
            y: self.y,
            z: self.x * angle.sin() + self.z * angle.cos(),
        }
    }

    pub fn rotate_z(&self, angle: f32) -> Self {
        Self {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
            z: self.z,
        }
    }
}

impl Add<PVec3> for PVec3 {
    type Output = PVec3;

    fn add(self, rhs: PVec3) -> Self::Output {
        PVec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<PVec3> for PVec3 {
    type Output = PVec3;

    fn sub(self, rhs: PVec3) -> Self::Output {
        PVec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for PVec3 {
    type Output = PVec3;

    fn mul(self, rhs: f32) -> Self::Output {
        PVec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

pub const fn pvec3(x: f32, y: f32, z: f32) -> PVec3 {
    PVec3 { x, y, z }
}
