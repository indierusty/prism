use macroquad::math::Vec3;

pub struct Light {
    direction: Vec3,
}

impl Light {
    pub fn new(direction: Vec3) -> Self {
        Self { direction }
    }
}
