use std::f32::consts::PI;

use macroquad::prelude::*;

use prism::canvas::{self, HEIGHT, PIXEL_SIZE, WIDTH};
use prism::light::Light;
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
    let jet_obj = std::fs::read_to_string("assets/f22.obj").unwrap();
    let mut jet = PMesh::from_obj(&jet_obj);
    jet.translation.z = 25.; // move a bit away from camera
    let camera = vec3(0., 0., 0.);
    let light = Light::new(vec3(0., 0., 1.));

    let half_width = (canvas::WIDTH / 2) as f32;
    let half_height = (canvas::HEIGHT / 2) as f32;

    let fov = PI / 3.; // 60 degress.
    let aspect_ratio = HEIGHT as f32 / WIDTH as f32;
    let z_near = 0.1;
    let z_far = 100.;
    let projection_matrix = Mat4::perspective_lh(fov, aspect_ratio, z_near, z_far);

    let mut draw_face = true;
    let mut draw_wireframe = true;
    let mut draw_normals = false;

    loop {
        clear_background(WHITE);
        canvas.clear(BLACK);

        if is_key_pressed(KeyCode::F) {
            draw_face = !draw_face;
        }
        if is_key_pressed(KeyCode::L) {
            draw_wireframe = !draw_wireframe;
        }
        if is_key_pressed(KeyCode::N) {
            draw_normals = !draw_normals;
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
            // jet.rotation.y += 1. * get_frame_time();
            jet.rotation.x += 0.5 * get_frame_time();
        }

        let scale_matrix = Mat4::from_scale(vec3(jet.scale.x, jet.scale.y, jet.scale.z));
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
            let color = t.3;

            let to_vec4 = |v: Vec3| vec4(v.x, v.y, v.z, 1.);

            //# Apply transformations
            let v1 = (world_matrix * to_vec4(v1)).xyz();
            let v2 = (world_matrix * to_vec4(v2)).xyz();
            let v3 = (world_matrix * to_vec4(v3)).xyz();

            //# Backface culling
            let v1v2 = (v2 - v1).normalize();
            let v1v3 = (v3 - v1).normalize();
            let normal = v1v2.cross(v1v3).normalize();
            let camera_ray = camera - v1;
            let dot = normal.dot(camera_ray);

            if dot < 0. {
                continue;
            }

            //# Apply flat shading.
            // Reverse direction of the light
            let light = light.direction().normalize() * -1.;
            // Calculate the % the normal and light pointing in the same direction [0, 1] using dot product.
            // Ignoring the negative values i.e when normal is pointing to opposite direction to the light.
            let intensity_factor = light.dot(normal).min(1.).max(0.);
            let color = Color {
                r: color.r * intensity_factor,
                g: color.g * intensity_factor,
                b: color.b * intensity_factor,
                a: color.a,
            };

            //# Perspective Projection.
            let project = |v: Vec4| {
                let v = projection_matrix * v;
                let v = v / v.w;
                v.xy() * half_height * 4. + vec2(half_width, half_height)
            };

            let v1 = project(to_vec4(v1));
            let v2 = project(to_vec4(v2));
            let v3 = project(to_vec4(v3));

            faces.push((v1, v2, v3, color));
            normal_rays.push((v1, normal));
        }

        if draw_face {
            for (v1, v2, v3, color) in &faces {
                canvas.draw_triangle(v1.xy(), v2.xy(), v3.xy(), *color);
            }
        }
        if draw_wireframe {
            for (v1, v2, v3, _) in &faces {
                canvas.draw_triangle_lines(v1.xy(), v2.xy(), v3.xy(), DARKGRAY);
            }
        }

        if draw_normals {
            for (vertex, normal) in normal_rays {
                let normal_ray = vertex.xy() + (normal.xy() * 10.);
                canvas.draw_line(vertex.xy(), normal_ray.xy(), RED);
            }
        }

        canvas.draw();
        next_frame().await
    }
}
