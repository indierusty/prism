use crate::cube::{CUBE_FACES, CUBE_VERTICES};
use crate::pvector::{PVec3, pvec3};

pub struct PMesh {
    pub vertices: Vec<PVec3>,
    pub indices: Vec<(usize, usize, usize)>,
    pub rotation: PVec3,
    pub scale: PVec3,
    pub translation: PVec3,
}

impl PMesh {
    pub fn cube() -> Self {
        Self {
            vertices: CUBE_VERTICES.to_vec(),
            indices: CUBE_FACES.to_vec(),
            rotation: PVec3::ZERO,
            scale: PVec3::ONE,
            translation: PVec3::ZERO,
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
                    vertices.push(pvec3(vertex[0], vertex[1], vertex[2]));
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
            rotation: PVec3::ZERO,
            scale: PVec3::ONE,
            translation: PVec3::ZERO,
        }
    }
}
