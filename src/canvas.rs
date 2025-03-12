use macroquad::prelude::*;

use crate::pvector::{PVec3, pvec3};

pub struct Canvas {
    pixels: Vec<Color>,
}

#[cfg(not(debug_assertions))]
pub const WIDTH: usize = 900;
#[cfg(not(debug_assertions))]
pub const PIXEL_SIZE: usize = 1;

#[cfg(debug_assertions)]
pub const WIDTH: usize = 300;
#[cfg(debug_assertions)]
pub const PIXEL_SIZE: usize = 3;

pub const HEIGHT: usize = WIDTH * 3 / 4;

impl Canvas {
    pub fn new() -> Self {
        Self {
            pixels: [WHITE; WIDTH * HEIGHT].to_vec(),
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [WHITE; WIDTH * HEIGHT].to_vec();
    }

    pub fn draw(&self) {
        let t = Texture2D::from_rgba8(
            WIDTH as u16,
            HEIGHT as u16,
            &self
                .pixels
                .iter()
                .flat_map(|c| Into::<[u8; 4]>::into(*c))
                .collect::<Vec<u8>>(),
        );
        draw_texture_ex(
            &t,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    WIDTH as f32 * PIXEL_SIZE as f32,
                    HEIGHT as f32 * PIXEL_SIZE as f32,
                )),
                ..Default::default()
            },
        );
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }
        self.pixels[(y * WIDTH) + x] = color;
    }

    pub fn draw_line(&mut self, start: PVec3, end: PVec3, color: Color) {
        let delta_x = end.x - start.x;
        let delta_y = end.y - start.y;

        let max_side_len = delta_x.abs().max(delta_y.abs());

        let x_inc = delta_x / max_side_len;
        let y_inc = delta_y / max_side_len;

        let mut cur_x = start.x;
        let mut cur_y = start.y;

        for _ in 0..=max_side_len as usize {
            self.set_pixel(cur_x as usize, cur_y as usize, color);
            cur_x += x_inc;
            cur_y += y_inc;
        }
    }

    pub fn draw_triangle(&mut self, mut v1: PVec3, mut v2: PVec3, mut v3: PVec3, color: Color) {
        // Sort the vertices based on y-axis in ascending order (v1.y <= v2.y <= v3.y).
        if v2.y < v1.y {
            std::mem::swap(&mut v1, &mut v2)
        }
        if v3.y < v1.y {
            std::mem::swap(&mut v1, &mut v3);
        }
        if v3.y < v2.y {
            std::mem::swap(&mut v2, &mut v3);
        }

        // TODO: make seperate fn and write test for this expression.
        // Calculate the point which divides the triangle into two where the line with endpoint `mid` and `v2` is the bisector.
        let mid_x = (v3.x - v1.x) * (v2.y - v1.y) / (v3.y - v1.y) + v1.x;
        let mut mid = pvec3(mid_x, v2.y, 0.);

        // `v2` vertex must be left to `mid` vertex required by draw_flat_bottom_triange and draw_flat_top_triangle function.
        if mid.x < v2.x {
            std::mem::swap(&mut mid, &mut v2);
        }

        self.draw_flat_bottom_triangle(v1, v2, mid, color);
        self.draw_flat_top_triangle(v2, mid, v3, color);
    }

    /// The `v2` & `v3` is the flat bottom side vertices.
    pub fn draw_flat_bottom_triangle(&mut self, v1: PVec3, v2: PVec3, v3: PVec3, color: Color) {
        let inv_slope_1 = (v2.x - v1.x) / (v2.y - v1.y);
        let inv_slope_2 = (v3.x - v1.x) / (v3.y - v1.y);

        let start_y = v1.y as usize;
        let end_y = v2.y as usize;

        let mut start_x = v1.x;
        let mut end_x = v1.x;

        for y in start_y..end_y {
            for x in start_x as usize..=end_x as usize {
                // Draw the horizontal line.
                self.set_pixel(x, y, color);
            }
            start_x += inv_slope_1;
            end_x += inv_slope_2;
        }
    }

    /// The `v1` & `v2` is the flat top side vertices.
    pub fn draw_flat_top_triangle(&mut self, v1: PVec3, v2: PVec3, v3: PVec3, color: Color) {
        let inv_slope_1 = (v3.x - v1.x) / (v3.y - v1.y);
        let inv_slope_2 = (v3.x - v2.x) / (v3.y - v2.y);

        let start_y = v1.y as usize;
        let end_y = v3.y as usize;

        let mut start_x = v1.x;
        let mut end_x = v2.x;

        for y in start_y..end_y {
            for x in start_x as usize..=end_x as usize {
                self.set_pixel(x, y, color);
            }
            start_x += inv_slope_1;
            end_x += inv_slope_2;
        }
    }
}
