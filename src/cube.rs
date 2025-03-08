use crate::pvector::{PVec3, pvec3};

#[rustfmt::skip]
pub const CUBE_VERTICES: [PVec3; 8] = [
    pvec3(-1., -1.,  1.),
    pvec3(-1.,  1.,  1.),
    pvec3( 1.,  1.,  1.),
    pvec3( 1., -1.,  1.),
    pvec3(-1., -1., -1.),
    pvec3(-1.,  1., -1.),
    pvec3( 1.,  1., -1.),
    pvec3( 1., -1., -1.),
];

pub const CUBE_FACES: [(usize, usize, usize); 12] = [
    // Front
    (0, 1, 2),
    (0, 2, 3),
    // Right
    (3, 2, 6),
    (3, 6, 7),
    // Back
    (7, 6, 5),
    (7, 5, 4),
    // Left
    (4, 5, 1),
    (4, 1, 0),
    // Top
    (1, 5, 6),
    (1, 6, 2),
    // Bottom
    (4, 7, 3),
    (4, 3, 0),
];
