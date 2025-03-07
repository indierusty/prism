use macroquad::prelude::*;

pub struct Canvas {
    pixels: [Color; WIDTH * HEIGHT],
}

pub const WIDTH: usize = 350;
pub const HEIGHT: usize = WIDTH * 3 / 4;
const PIXEL_SIZE: f32 = 2.;

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
