use macroquad::prelude::*;

pub struct Canvas {
    pixels: [Color; WIDTH * HEIGHT],
}

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = WIDTH * 3 / 4;
const PIXEL_SIZE: f32 = 3.;

impl Canvas {
    pub fn new() -> Self {
        Self {
            pixels: [WHITE; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [WHITE; WIDTH * HEIGHT];
    }

    pub fn draw(&self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let p = vec2(x as f32, y as f32) * PIXEL_SIZE;
                let c = self.pixels[(x * HEIGHT) + y];
                draw_rectangle(p.x, p.y, PIXEL_SIZE, PIXEL_SIZE, c);
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }
        self.pixels[(x * HEIGHT) + y] = color;
    }

    pub fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color) {
        let delta_x = end.x - start.x;
        let delta_y = end.y - start.y;

        let max_side_len = delta_x.abs().max(delta_y.abs());

        let mut i = 0.;
        loop {
            if i >= max_side_len {
                break;
            }
            let x = i * (delta_x / max_side_len) + start.x;
            let y = i * (delta_y / max_side_len) + start.y;
            self.set_pixel(x as usize, y as usize, color);
            i += 1.;
        }
    }
}
