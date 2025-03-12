use macroquad::prelude::*;

use prism::canvas::{self, HEIGHT, PIXEL_SIZE, WIDTH};
use prism::pmatrix::PMat4;
use prism::pmesh::PMesh;
use prism::pvector::{PVec3, PVec4, pvec3};

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
    let mut scale = 5.;
    let jet_obj = std::fs::read_to_string("assets/f22.obj").unwrap();
    let mut jet = PMesh::from_obj(&jet_obj);
    let camera = pvec3(0., 0., 0.);
    let fov = 320.;

    let half_width = (canvas::WIDTH / 2) as f32;
    let half_height = (canvas::HEIGHT / 2) as f32;

    loop {
        canvas.clear();

        if is_key_down(KeyCode::K) {
            scale += 25. * get_frame_time();
        } else if is_key_down(KeyCode::J) {
            scale -= 25. * get_frame_time();
        }
        if is_key_down(KeyCode::A) {
            jet.translation.x -= 25. * get_frame_time();
        } else if is_key_down(KeyCode::D) {
            jet.translation.x += 25. * get_frame_time();
        } else if is_key_down(KeyCode::W) {
            jet.translation.y -= 25. * get_frame_time();
        } else if is_key_down(KeyCode::S) {
            jet.translation.y += 25. * get_frame_time();
        }

        if !is_key_down(KeyCode::Space) {
            jet.rotation.x += 1. * get_frame_time();
        }

        let scale_matrix = PMat4::from_scale(scale, scale, scale);
        let translation_matrix =
            PMat4::from_translation(jet.translation.x, jet.translation.y, jet.translation.z);
        let rotation_matrix_x = PMat4::from_rotation_x(jet.rotation.x);
        let rotation_matrix_y = PMat4::from_rotation_y(jet.rotation.y);
        let rotation_matrix_z = PMat4::from_rotation_z(jet.rotation.z);

        let world_matrix = translation_matrix
            * rotation_matrix_x
            * rotation_matrix_y
            * rotation_matrix_z
            * scale_matrix;

        for t in &jet.indices {
            let a = jet.vertices[t.0 - 1];
            let b = jet.vertices[t.1 - 1];
            let c = jet.vertices[t.2 - 1];

            // Transforming the vertices.
            let triangle = [a, b, c]
                .iter()
                .map(|v| v.rotate_x(jet.rotation.x))
                .map(|v| PVec4::from(v))
                .map(|v| world_matrix * v) // scale the mesh
                .map(|v| pvec3(v.x, v.y, v.z))
                .map(|v| v - pvec3(0., 0., 25.)) // move the mesh away from the camera
                .collect::<Vec<PVec3>>();

            // Backface culling
            let a_to_b = triangle[1] - triangle[0];
            let a_to_c = triangle[2] - triangle[0];
            let normal = a_to_b.normalize().cross(a_to_c.normalize()).normalize();
            let vertex_a_to_camera = camera - triangle[0];
            let dot = normal.dot(vertex_a_to_camera);

            // backface culling.
            if dot < 0. {
                continue;
            }

            // Projecting the vertices.
            let triangle = triangle
                .into_iter()
                .map(|v| pvec3((v.x * fov) / v.z, (v.y * fov) / v.z, v.z)) // project
                .map(|v| v + pvec3(half_width, half_height, 0.)) // translate the mesh to move in mid on screen
                .collect::<Vec<PVec3>>();

            canvas.draw_triangle(triangle[0], triangle[1], triangle[2], GRAY);

            canvas.draw_line(triangle[0], triangle[1], DARKGRAY);
            canvas.draw_line(triangle[1], triangle[2], DARKGRAY);
            canvas.draw_line(triangle[2], triangle[0], DARKGRAY);
        }

        canvas.draw();
        next_frame().await
    }
}
