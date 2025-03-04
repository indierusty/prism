use macroquad::prelude::*;

use prism::canvas;

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
    let mut canvas = canvas::Canvas::new();
    let mut angle = 0.;

    loop {
        clear_background(BLACK);
        canvas.clear();

        let hw = (canvas::WIDTH / 2) as f32;
        let hh = (canvas::HEIGHT / 2) as f32;

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
