use std::f32::consts::PI;

use macroquad::prelude::*;

use prism::canvas::{self, HEIGHT, PIXEL_SIZE, WIDTH};
use prism::pmesh::PMesh;

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
    let camera = vec3(0., 0., -1100.);

    let half_width = (canvas::WIDTH / 2) as f32;
    let half_height = (canvas::HEIGHT / 2) as f32;

    let fov = PI / 5.;
    let aspect_ratio = HEIGHT as f32 / WIDTH as f32;
    let z_near = 0.1;
    let z_far = 100.;
    let prespective_matrix = Mat4::perspective_lh(fov, aspect_ratio, z_near, z_far);

    println!("jet ==> {:?}", jet.indices.len());

    loop {
        clear_background(WHITE);
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

        let scale_matrix = Mat4::from_scale(vec3(scale, scale, scale));
        let translation_matrix = Mat4::from_translation(vec3(
            jet.translation.x,
            jet.translation.y,
            jet.translation.z,
        ));
        let rotation_matrix_x = Mat4::from_rotation_x(jet.rotation.x);
        let rotation_matrix_y = Mat4::from_rotation_y(jet.rotation.y);
        let rotation_matrix_z = Mat4::from_rotation_z(jet.rotation.z);

        let world_matrix = translation_matrix
            * rotation_matrix_x
            * rotation_matrix_y
            * rotation_matrix_z
            * scale_matrix;

        let mut faces = Vec::new();
        let mut normal_rays = Vec::new();

        for t in &jet.indices {
            let v1 = jet.vertices[t.0 - 1];
            let v2 = jet.vertices[t.1 - 1];
            let v3 = jet.vertices[t.2 - 1];

            // Transforming the vertices.
            let ts = |v: Vec3| (world_matrix * vec4(v.x, v.y, v.z, 1.)).xyz();

            let (v1, v2, v3) = (ts(v1), ts(v2), ts(v3));

            // Backface culling
            let a_to_b = (v2 - v1).normalize();
            let a_to_c = (v3 - v1).normalize();
            let normal = a_to_b.cross(a_to_c).normalize();
            let camera_ray = camera - v1;
            let dot = normal.dot(camera_ray);

            // backface culling.
            if dot > 0. {
                continue;
            }

            let project = |v: Vec3| {
                prespective_matrix * vec4(v.x, v.y, v.z, 1.) + vec4(half_width, half_height, 0., 0.)
            };
            // Projecting the vertices.
            let (v1, v2, v3) = (project(v1).xyz(), project(v2).xyz(), project(v3).xyz());

            normal_rays.push((v1, normal));
            faces.push((v1, v2, v3))
        }

        for (v1, v2, v3) in &faces {
            canvas.draw_triangle(v1.xy(), v2.xy(), v3.xy(), GRAY);
        }
        for (v1, v2, v3) in &faces {
            canvas.draw_triangle_lines(v1.xy(), v2.xy(), v3.xy(), DARKGRAY);
        }

        for (vertex, normal) in normal_rays {
            let normal_ray = vertex + (normal * scale);
            canvas.draw_line(vertex.xy(), normal_ray.xy(), RED);
        }

        canvas.draw();
        next_frame().await
    }
}
