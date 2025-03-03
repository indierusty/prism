use macroquad::prelude::*;

pub struct Canvas {
    pixels: [Color; Self::WIDTH * Self::HEIGHT],
}

impl Canvas {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256 * 3 / 4;
    const PIXEL_SIZE: f32 = 3.;

    pub fn new() -> Self {
        Self {
            pixels: [WHITE; Self::WIDTH * Self::HEIGHT],
        }
    }

    pub fn draw(&self) {
        for x in 0..Self::WIDTH {
            for y in 0..Self::HEIGHT {
                let p = vec2(x as f32, y as f32) * Self::PIXEL_SIZE;
                let c = self.pixels[(x * Self::HEIGHT) + y];
                draw_rectangle(p.x, p.y, Self::PIXEL_SIZE, Self::PIXEL_SIZE, c);
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[(x * Self::HEIGHT) + y] = color;
    }
}

#[macroquad::main("Prism")]
async fn main() {
    let mut pixels = Canvas::new();

    loop {
        clear_background(BLACK);
        pixels.draw();

        for x in (0..150).step_by(2) {
            for y in (0..150).step_by(2) {
                pixels.set_pixel(x, y, BLUE);
            }
        }
        next_frame().await
    }
}
