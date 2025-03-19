use macroquad::prelude::{Vec3, vec3};

#[rustfmt::skip]
pub const CUBE_VERTICES: [Vec3; 8] = [
    vec3(-1.,  1.,  1.),
    vec3(-1., -1.,  1.),
    vec3( 1.,  1.,  1.),
    vec3( 1., -1.,  1.),
    vec3(-1., -1., -1.),
    vec3(-1.,  1., -1.),
    vec3( 1.,  1., -1.),
    vec3( 1., -1., -1.),
];

pub const CUBE_FACES: [(usize, usize, usize); 12] = [
    // Front
    (1, 2, 3),
    (1, 3, 4),
    // Right
    (4, 3, 7),
    (4, 7, 8),
    // Back
    (8, 7, 6),
    (8, 6, 5),
    // Left
    (5, 6, 2),
    (5, 2, 1),
    // Top
    (2, 6, 7),
    (2, 7, 3),
    // Bottom
    (5, 8, 4),
    (5, 4, 1),
];
