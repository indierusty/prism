use macroquad::prelude::*;

#[rustfmt::skip]
pub const CUBE_VERTICES: [Vec3; 8] = [
    vec3(-1., -1., -1.),
    vec3(-1.,  1., -1.),
    vec3( 1.,  1., -1.),
    vec3( 1., -1., -1.),
    vec3(-1., -1.,  1.),
    vec3(-1.,  1.,  1.),
    vec3( 1.,  1.,  1.),
    vec3( 1., -1.,  1.),
];

pub const CUBE_FACES: [(usize, usize, usize); 12] = [
    // Front
    (1, 2, 3),
    (1, 3, 4),
    // Right
    (4, 3, 5),
    (4, 5, 6),
    // Back
    (6, 5, 7),
    (6, 7, 8),
    // Left
    (8, 7, 2),
    (8, 2, 1),
    // Top
    (2, 7, 5),
    (2, 5, 3),
    // Bottom
    (6, 8, 1),
    (6, 1, 4),
];
