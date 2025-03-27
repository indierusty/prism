use macroquad::{
    color::{Color, WHITE},
    prelude::{Vec3, vec3},
};

#[rustfmt::skip]
pub const CUBE_VERTICES: [Vec3; 8] = [
    vec3(-1., -1.,  1.),
    vec3( 1., -1.,  1.),
    vec3(-1.,  1.,  1.),
    vec3( 1.,  1.,  1.),
    vec3(-1.,  1., -1.),
    vec3( 1.,  1., -1.),
    vec3(-1., -1., -1.),
    vec3( 1., -1., -1.),
];

pub const CUBE_FACES: [(usize, usize, usize, Color); 12] = [
    // Front
    (1, 2, 3, WHITE),
    (3, 2, 4, WHITE),
    // Right
    (3, 4, 5, WHITE),
    (5, 4, 6, WHITE),
    // Back
    (5, 6, 7, WHITE),
    (7, 6, 8, WHITE),
    // Left
    (7, 8, 1, WHITE),
    (1, 8, 2, WHITE),
    // Top
    (2, 8, 4, WHITE),
    (4, 8, 6, WHITE),
    // Bottom
    (7, 1, 5, WHITE),
    (5, 1, 3, WHITE),
];
