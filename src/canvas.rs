use macroquad::prelude::*;

use crate::pvector::PVec3;

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

        for _ in 0..max_side_len as usize {
            self.set_pixel(cur_x as usize, cur_y as usize, color);
            cur_x += x_inc;
            cur_y += y_inc;
        }
    }
}
