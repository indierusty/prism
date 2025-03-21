use std::ops::{Add, Div, Mul, Sub};

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

    pub const ONE: Self = Self {
        x: 1.,
        y: 1.,
        z: 1.,
    };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn rotate_x(self, angle: f32) -> Self {
        Self {
            x: self.x,
            y: self.y * angle.cos() - self.z * angle.sin(),
            z: self.y * angle.sin() + self.z * angle.cos(),
        }
    }

    pub fn rotate_y(self, angle: f32) -> Self {
        Self {
            x: self.x * angle.cos() - self.z * angle.sin(),
            y: self.y,
            z: self.x * angle.sin() + self.z * angle.cos(),
        }
    }

    pub fn rotate_z(self, angle: f32) -> Self {
        Self {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
            z: self.z,
        }
    }

    pub fn length(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl Add<PVec3> for PVec3 {
    type Output = PVec3;

    fn add(self, rhs: PVec3) -> Self::Output {
        PVec3 {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}

impl Sub<PVec3> for PVec3 {
    type Output = PVec3;

    fn sub(self, rhs: PVec3) -> Self::Output {
        PVec3 {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}

impl Mul<f32> for PVec3 {
    type Output = PVec3;

    fn mul(self, rhs: f32) -> Self::Output {
        PVec3 {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}

impl Div<f32> for PVec3 {
    type Output = PVec3;

    fn div(self, rhs: f32) -> Self::Output {
        PVec3 {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
        }
    }
}

pub const fn pvec3(x: f32, y: f32, z: f32) -> PVec3 {
    PVec3 { x, y, z }
}

#[derive(Clone, Copy, Debug)]
pub struct PVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<PVec3> for PVec4 {
    fn from(value: PVec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: 1.,
        }
    }
}

pub const fn pvec4(x: f32, y: f32, z: f32, w: f32) -> PVec4 {
    PVec4 { x, y, z, w }
}
