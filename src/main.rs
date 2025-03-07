use macroquad::prelude::*;

use prism::{
    canvas,
    cube::{CUBE_FACES, CUBE_VERTICES},
};

#[macroquad::main("Prism")]
async fn main() {
    let mut canvas = canvas::Canvas::new();
    let mut angle = 0.;
    let mut scale = 50.;

    loop {
        clear_background(BLACK);
        canvas.clear();

        if is_key_down(KeyCode::K) {
            scale += 0.5;
        } else if is_key_down(KeyCode::J) {
            scale -= 0.5;
        }

        let hw = (canvas::WIDTH / 2) as f32;
        let hh = (canvas::HEIGHT / 2) as f32;

        if !is_key_down(KeyCode::Space) {
            angle += get_frame_time();
        }

        for t in CUBE_FACES {
            let a = CUBE_VERTICES[t.0];
            let b = CUBE_VERTICES[t.1];
            let c = CUBE_VERTICES[t.2];

            let triangle = [a, b, c]
                .iter()
                .map(|v| Mat3::from_rotation_y(angle) * *v)
                .map(|v| vec2(v.x / (v.z + 3.), v.y / (v.z + 3.)))
                .map(|v| vec2(v.x * scale + hw, v.y * scale + hh))
                .collect::<Vec<Vec2>>();

            canvas.draw_line(triangle[0], triangle[1], DARKGRAY);
            canvas.draw_line(triangle[1], triangle[2], DARKGRAY);
            canvas.draw_line(triangle[2], triangle[0], DARKGRAY);
        }

        canvas.draw();
        next_frame().await
    }
}
