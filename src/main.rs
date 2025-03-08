use macroquad::prelude::*;

use prism::canvas::{self, HEIGHT, PIXEL_SIZE, WIDTH};
use prism::pmesh::PMesh;
use prism::pvector::{PVec3, pvec3};

fn conf() -> Conf {
    Conf {
        window_title: "Prism".to_string(),
        window_width: (WIDTH * PIXEL_SIZE) as i32,
        window_height: (HEIGHT * PIXEL_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut canvas = canvas::Canvas::new();
    let mut scale = 50.;
    let mut cube = PMesh::cube();

    loop {
        canvas.clear();

        if is_key_down(KeyCode::K) {
            scale += 45. * get_frame_time();
        } else if is_key_down(KeyCode::J) {
            scale -= 45. * get_frame_time();
        }

        let hw = (canvas::WIDTH / 2) as f32;
        let hh = (canvas::HEIGHT / 2) as f32;

        if !is_key_down(KeyCode::Space) {
            cube.rotation.y += 2. * get_frame_time();
        }

        for t in &cube.indices {
            let a = cube.vertices[t.0];
            let b = cube.vertices[t.1];
            let c = cube.vertices[t.2];

            let triangle = [a, b, c]
                .iter()
                .map(|v| v.rotate_y(cube.rotation.y))
                .map(|v| pvec3(v.x / (v.z + 3.), v.y / (v.z + 3.), v.z))
                .map(|v| pvec3(v.x * scale + hw, v.y * scale + hh, v.z))
                .collect::<Vec<PVec3>>();

            canvas.draw_line(triangle[0], triangle[1], DARKGRAY);
            canvas.draw_line(triangle[1], triangle[2], DARKGRAY);
            canvas.draw_line(triangle[2], triangle[0], DARKGRAY);
        }

        canvas.draw();
        next_frame().await
    }
}
