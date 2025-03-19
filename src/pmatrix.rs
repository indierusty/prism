use std::ops::Mul;

use crate::pvector::PVec4;

#[derive(Clone, Copy, Debug)]
pub struct PMat4 {
    m: [[f32; 4]; 4],
}

impl PMat4 {
    pub const ZERO: Self = Self { m: [[0.; 4]; 4] };

    pub fn new(m: [[f32; 4]; 4]) -> Self {
        Self { m }
    }

    pub const fn identity() -> Self {
        Self {
            m: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn from_scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            #[rustfmt::skip]
            m: [
                [x , 0., 0., 0.],
                [0., y , 0., 0.],
                [0., 0., z , 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn from_translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            #[rustfmt::skip]
            m: [
                [1., 0., 0., x ],
                [0., 1., 0., y ],
                [0., 0., 1., z ],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn from_rotation_z(angle: f32) -> Self {
        let cosa = angle.cos();
        let sina = angle.sin();

        Self {
            #[rustfmt::skip]
            m: [
                [ cosa,-sina,   0.,   0.],
                [ sina, cosa,   0.,   0.],
                [   0.,   0.,   1.,   0.],
                [   0.,   0.,   0.,   1.],
            ],
        }
    }

    pub fn from_rotation_x(angle: f32) -> Self {
        let cosa = angle.cos();
        let sina = angle.sin();

        Self {
            #[rustfmt::skip]
            m: [
                [   1.,   0.,   0.,   0.],
                [   0., cosa,-sina,   0.],
                [   0., sina, cosa,   0.],
                [   0.,   0.,   0.,   1.],
            ],
        }
    }

    pub fn from_rotation_y(angle: f32) -> Self {
        let cosa = angle.cos();
        let sina = angle.sin();

        Self {
            #[rustfmt::skip]
            m: [
                [ cosa,   0., sina,   0.],
                [   0.,   1.,   0.,   0.],
                [-sina,   0., cosa,   0.],
                [   0.,   0.,   0.,   1.],
            ],
        }
    }

    pub fn make_perspective(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Self {
        let mut identity = Self::ZERO;
        identity.m[0][0] = aspect * (1. / (fov / 2.).tan());
        identity.m[1][1] = 1. / (fov / 2.).tan();
        identity.m[2][2] = zfar / (zfar - znear);
        identity.m[2][3] = (-zfar * znear) / (zfar - znear);
        identity.m[3][3] = 1.0; // to record the z value of the vector in w value.
        identity
    }
}

pub fn project_vec4(projection_mat: PMat4, vertex: PVec4) -> PVec4 {
    let mut res = projection_mat * vertex;

    if res.w != 0. {
        res.x /= res.w;
        res.y /= res.w;
        res.z /= res.w;
    }

    res
}

impl Mul<PVec4> for PMat4 {
    type Output = PVec4;

    fn mul(self, rhs: PVec4) -> Self::Output {
        let m = self.m;
        PVec4 {
            x: m[0][0] * rhs.x + m[0][1] * rhs.y + m[0][2] * rhs.z + m[0][3] * rhs.w,
            y: m[1][0] * rhs.x + m[1][1] * rhs.y + m[1][2] * rhs.z + m[1][3] * rhs.w,
            z: m[2][0] * rhs.x + m[2][1] * rhs.y + m[2][2] * rhs.z + m[2][3] * rhs.w,
            w: m[3][0] * rhs.x + m[3][1] * rhs.y + m[3][2] * rhs.z + m[3][3] * rhs.w,
        }
    }
}

impl Mul<PMat4> for PMat4 {
    type Output = Self;

    fn mul(self, rhs: PMat4) -> Self::Output {
        let lm = self.m;
        let rm = rhs.m;
        Self {
            #[rustfmt::skip]
            m:
            [
                [
                    lm[0][0] * rm[0][0] + lm[0][1] * rm[1][0] + lm[0][2] * rm[2][0] + lm[0][3] * rm[3][0],
                    lm[0][0] * rm[0][1] + lm[0][1] * rm[1][1] + lm[0][2] * rm[2][1] + lm[0][3] * rm[3][1],
                    lm[0][0] * rm[0][2] + lm[0][1] * rm[1][2] + lm[0][2] * rm[2][2] + lm[0][3] * rm[3][2],
                    lm[0][0] * rm[0][3] + lm[0][1] * rm[1][3] + lm[0][2] * rm[2][3] + lm[0][3] * rm[3][3],
                ],
                [
                    lm[1][0] * rm[0][0] + lm[1][1] * rm[1][0] + lm[1][2] * rm[2][0] + lm[1][3] * rm[3][0],
                    lm[1][0] * rm[0][1] + lm[1][1] * rm[1][1] + lm[1][2] * rm[2][1] + lm[1][3] * rm[3][1],
                    lm[1][0] * rm[0][2] + lm[1][1] * rm[1][2] + lm[1][2] * rm[2][2] + lm[1][3] * rm[3][2],
                    lm[1][0] * rm[0][3] + lm[1][1] * rm[1][3] + lm[1][2] * rm[2][3] + lm[1][3] * rm[3][3],
                ],
                [
                    lm[2][0] * rm[0][0] + lm[2][1] * rm[1][0] + lm[2][2] * rm[2][0] + lm[2][3] * rm[3][0],
                    lm[2][0] * rm[0][1] + lm[2][1] * rm[1][1] + lm[2][2] * rm[2][1] + lm[2][3] * rm[3][1],
                    lm[2][0] * rm[0][2] + lm[2][1] * rm[1][2] + lm[2][2] * rm[2][2] + lm[2][3] * rm[3][2],
                    lm[2][0] * rm[0][3] + lm[2][1] * rm[1][3] + lm[2][2] * rm[2][3] + lm[2][3] * rm[3][3],
                ],
                [
                    lm[3][0] * rm[0][0] + lm[3][1] * rm[1][0] + lm[3][2] * rm[2][0] + lm[3][3] * rm[3][0],
                    lm[3][0] * rm[0][1] + lm[3][1] * rm[1][1] + lm[3][2] * rm[2][1] + lm[3][3] * rm[3][1],
                    lm[3][0] * rm[0][2] + lm[3][1] * rm[1][2] + lm[3][2] * rm[2][2] + lm[3][3] * rm[3][2],
                    lm[3][0] * rm[0][3] + lm[3][1] * rm[1][3] + lm[3][2] * rm[2][3] + lm[3][3] * rm[3][3],
                ],
            ],
        }
    }
}
