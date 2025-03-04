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

    pub fn clear(&mut self) {
        self.pixels = [WHITE; Self::WIDTH * Self::HEIGHT];
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
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            return;
        }
        self.pixels[(x * Self::HEIGHT) + y] = color;
    }
}

fn cube() -> Vec<Vec3> {
    let mut vertices = Vec::new();
    for x in 0..=10 {
        for y in 0..=10 {
            for z in 0..=10 {
                let (x, y, z) = (x - 5, y - 5, z - 5);
                vertices.push(vec3(x as f32, y as f32, z as f32));
            }
        }
    }
    vertices
}

#[macroquad::main("Prism")]
async fn main() {
    let mut canvas = Canvas::new();
    let mut angle = 0.;

    loop {
        clear_background(BLACK);
        canvas.clear();

        let hw = (Canvas::WIDTH / 2) as f32;
        let hh = (Canvas::HEIGHT / 2) as f32;

        let cube = cube()
            .iter()
            .map(|v| Mat3::from_rotation_y(angle) * *v)
            .map(|v| vec2(v.x / (v.z + 10.), v.y / (v.z + 10.)))
            .map(|v| vec2(v.x * 50. + hw, v.y * 50. + hh))
            .collect::<Vec<Vec2>>();

        angle += 0.05;

        for v in cube {
            canvas.set_pixel(v.x as usize, v.y as usize, RED);
        }
        canvas.draw();
        next_frame().await
    }
}
