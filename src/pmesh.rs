use crate::cube::{CUBE_FACES, CUBE_VERTICES};

use macroquad::prelude::{Vec3, vec3};

pub struct PMesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<(usize, usize, usize)>,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub translation: Vec3,
}

impl PMesh {
    pub fn cube() -> Self {
        Self {
            vertices: CUBE_VERTICES.to_vec(),
            indices: CUBE_FACES.to_vec(),
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            translation: Vec3::ZERO,
        }
    }

    pub fn from_obj(src: &str) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for line in src.lines() {
            let mut words = line.split_whitespace();
            match words.next() {
                Some("v") => {
                    let vertex = words
                        .map(|v| v.parse::<f32>().unwrap())
                        .collect::<Vec<f32>>();
                    vertices.push(vec3(vertex[0], vertex[1], vertex[2]));
                }
                Some("f") => {
                    let face = words
                        .map(|w| {
                            w.split('/')
                                .map(|n| n.parse::<usize>().unwrap())
                                .next()
                                .unwrap()
                        })
                        .collect::<Vec<usize>>();
                    indices.push((face[0], face[1], face[2]));
                }
                _ => {}
            }
        }

        Self {
            vertices,
            indices,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            translation: Vec3::ZERO,
        }
    }
}
