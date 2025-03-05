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
    (0, 1, 2),
    (0, 2, 3),
    // Right
    (3, 2, 4),
    (3, 4, 5),
    // Back
    (5, 4, 6),
    (5, 6, 7),
    // Left
    (7, 6, 1),
    (7, 1, 0),
    // Top
    (1, 6, 4),
    (1, 4, 2),
    // Bottom
    (5, 7, 0),
    (5, 0, 3),
];
