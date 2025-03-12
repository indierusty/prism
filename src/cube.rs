use crate::pvector::{PVec3, pvec3};

#[rustfmt::skip]
pub const CUBE_VERTICES: [PVec3; 8] = [
    pvec3(-1., -1.,  1.),
    pvec3( 1., -1.,  1.),
    pvec3(-1.,  1.,  1.),
    pvec3( 1.,  1.,  1.),
    pvec3(-1.,  1., -1.),
    pvec3( 1.,  1., -1.),
    pvec3(-1., -1., -1.),
    pvec3( 1., -1., -1.),
];

pub const CUBE_FACES: [(usize, usize, usize); 12] = [
    // Front
    (1, 2, 3),
    (3, 2, 4),
    // Right
    (3, 4, 5),
    (5, 4, 6),
    // Back
    (5, 6, 7),
    (7, 6, 8),
    // Left
    (7, 8, 1),
    (1, 8, 2),
    // Top
    (2, 8, 4),
    (4, 8, 6),
    // Bottom
    (7, 1, 5),
    (5, 1, 3),
];
