use crate::cube::{CUBE_FACES, CUBE_VERTICES};
use crate::pvector::PVec3;

pub struct PMesh {
    pub vertices: Vec<PVec3>,
    pub indices: Vec<(usize, usize, usize)>,
    pub rotation: PVec3,
}

impl PMesh {
    pub fn cube() -> Self {
        Self {
            vertices: CUBE_VERTICES.to_vec(),
            indices: CUBE_FACES.to_vec(),
            rotation: PVec3::ZERO,
        }
    }
}
