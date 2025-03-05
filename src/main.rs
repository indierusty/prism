use macroquad::prelude::*;

use prism::{
    canvas,
    cube::{CUBE_FACES, CUBE_VERTICES},
};

#[macroquad::main("Prism")]
async fn main() {
    let mut canvas = canvas::Canvas::new();
    let mut angle = 0.;

    loop {
        clear_background(BLACK);
        canvas.clear();

        let hw = (canvas::WIDTH / 2) as f32;
        let hh = (canvas::HEIGHT / 2) as f32;

        angle += 0.05;

        for t in CUBE_FACES {
            let a = CUBE_VERTICES[t.0];
            let b = CUBE_VERTICES[t.1];
            let c = CUBE_VERTICES[t.2];

            let triangle = [a, b, c]
                .iter()
                .map(|v| Mat3::from_rotation_y(angle) * *v)
                .map(|v| vec2(v.x / (v.z + 3.), v.y / (v.z + 3.)))
                .map(|v| vec2(v.x * 50. + hw, v.y * 50. + hh))
                .collect::<Vec<Vec2>>();

            for vertex in triangle {
                canvas.set_pixel(vertex.x as usize, vertex.y as usize, RED);
            }
        }

        canvas.draw();
        next_frame().await
    }
}
