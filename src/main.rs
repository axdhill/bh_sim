use macroquad::prelude::*;
mod bhtree;
mod util;
use crate::util::Object;
use image::{ImageBuffer, Rgba};
use std::fs;

const G: f32 = 6.6743E-6; // N * m^2 / kg^2
const PIXEL_DENSITY: f32 = 600.0;
const HEIGHT: i32 = 600;
const WIDTH: i32 = 800;

fn window_conf() -> Conf {
    Conf {
        window_title: "Square Window".to_string(),
        window_width: WIDTH,   // Set width to desired square size
        window_height: HEIGHT, // Set height equal to width for a square
        fullscreen: false,
        ..Default::default()
    }
}

fn space_to_screen(pos: Vec2) -> Vec2 {
    let ratio = 2.0;
    let x = (pos.x + ratio / 2.0 * (WIDTH as f32) / (HEIGHT as f32)) * PIXEL_DENSITY / ratio;
    let y = (pos.y + ratio / 2.0) * PIXEL_DENSITY / ratio;
    vec2(x, y)
}
fn distance_sq(obj1: &Object, obj2: &Object) -> f32 {
    return (obj1.pos.x - obj2.pos.x).powf(2.0) + (obj1.pos.y - obj2.pos.y).powf(2.0);
}

fn force_between(obj1: &Object, obj2: &Object) -> Vec2 {
    let mut mag = G * obj1.mass * obj2.mass / distance_sq(obj1, obj2);
    if mag > 0.01 {
        mag = 0.01;
    }
    let dir = (obj1.pos - obj2.pos).normalize();
    return -mag * dir;
}

fn draw_arrow(origin: Vec2, displacement: Vec2) {
    draw_line(
        origin.x,
        origin.y,
        origin.x + displacement.x,
        origin.y + displacement.y,
        1.0,
        WHITE,
    )
}

fn in_screen(x: Vec2) -> bool {
    return (x.x > 0.0) & (x.y > 0.0) & (x.x < WIDTH as f32) & (x.y < HEIGHT as f32);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut objects = Vec::new();
    let n_objects = 200;

    let dt = 0.001;

    for _j in 0..n_objects {
        let mut color = WHITE;

        let _pos = vec2(rand::gen_range(-0.5, 0.5), rand::gen_range(-0.5, 0.5));
        let _t = _pos.y.atan2(_pos.x);
        if _t > 1.0 {
            color = Color::new(0.9296875, 0.5078125, 0.9296875, 1.0);
        }
        let o = Object {
            mass: rand::gen_range(0.25, 1.0),
            pos: _pos,
            vel: (_pos.y.powf(2.0) + _pos.x.powf(2.0)).sqrt() * vec2(-_t.sin(), _t.cos()) / 5.0,
            color: color,
        };
        println!("{:?}", o);
        objects.push(o);
    }

    // objects.push(Object { mass: 10.0, pos: vec2(-0.05, 0.0), vel: vec2(0.0,0.01)});
    // objects.push(Object { mass: 10.0, pos: vec2(0.05, 0.0), vel: vec2(0.0,-0.01)});

    let mut frame_count = 0;
    let mut iter_count = 0;
    let mut max_force = 0.0;

    let mut grabbed = true;
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    loop {
        clear_background(BLACK);

        // let mouse_position: Vec2 = mouse_position().into();
        // let r = 1.0;
        // let x = r * mouse_position.x.sin() * mouse_position.y.cos();
        // let y = r * mouse_position.x.sin() * mouse_position.y.sin();
        // let z = r * mouse_position.x.cos();

        // set_camera(&Camera3D {
        //     position: vec3(x,y,z),
        //     up: vec3(0.0, 0.0, 1.0),
        //     target: vec3(0.0, 0.0, 0.0),
        //     ..Default::default()
        // });

        for i in 0..objects.len() {
            let pos_s = space_to_screen(objects[i].pos);
            let c = objects[i].color;
            if in_screen(pos_s) {
                let size = (objects[i].mass).sqrt();
                draw_circle(
                    pos_s.x,
                    pos_s.y,
                    size,
                    Color {
                        r: c.r,
                        g: c.g,
                        b: c.b,
                        a: 0.5,
                    },
                );
                draw_circle(
                    pos_s.x,
                    pos_s.y,
                    2.0 * size,
                    Color {
                        r: c.r,
                        g: c.g,
                        b: c.b,
                        a: 0.25,
                    },
                );
                draw_circle(
                    pos_s.x,
                    pos_s.y,
                    4.0 * size,
                    Color {
                        r: c.r,
                        g: c.g,
                        b: c.b,
                        a: 0.125,
                    },
                );
            }
        }

        set_default_camera();
        // let image = get_screen_data();

        // if iter_count % 10 == 0 {
        //     image.export_png(&format!("frames/frame_{:05}.png", frame_count));
        //     frame_count += 1;
        // }
        // iter_count += 1;

        // draw_texture_ex(
        //     &render_target.texture,
        //     0.,
        //     0.,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(vec2(screen_width(), screen_height())),
        //         ..Default::default()
        //     },
        // );

        for i in 0..objects.len() {
            let mut force = vec2(0.0, 0.0);

            for j in 0..objects.len() {
                if j != i {
                    force += force_between(&objects[i], &objects[j]);
                }
            }
            // draw_arrow(space_to_screen(objects[i].pos), 1000.0*force);

            objects[i].vel.x += force.x / objects[i].mass * dt;
            objects[i].vel.y += force.y / objects[i].mass * dt;

            objects[i].pos.x += objects[i].vel.x * dt;
            objects[i].pos.y += objects[i].vel.y * dt;
        }

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to compare Vec2 with some tolerance (due to floating-point precision)
    fn vec2_approx_eq(a: Vec2, b: Vec2, tol: f32) -> bool {
        (a.x - b.x).abs() < tol && (a.y - b.y).abs() < tol
    }

    #[test]
    fn test_distance_sq() {
        // Test distance squared between two objects
        let obj1 = Object {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
            mass: 1.0,
            color: WHITE,
        };
        let obj2 = Object {
            pos: Vec2::new(3.0, 4.0),
            vel: Vec2::new(0.0, 0.0),
            mass: 1.0,
            color: WHITE,
        };
        let expected_distance_sq = 25.0; // (3^2 + 4^2)
        let result = distance_sq(&obj1, &obj2);
        assert_eq!(result, expected_distance_sq);
    }

    #[test]
    fn test_force_between() {
        // Test force calculation between two objects
        let obj1 = Object {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
            mass: 5.0,
            color: WHITE,
        };
        let obj2 = Object {
            pos: Vec2::new(3.0, 4.0),
            vel: Vec2::new(0.0, 0.0),
            mass: 10.0,
            color: WHITE,
        };
        let dist_sq = distance_sq(&obj1, &obj2);
        let expected_force_mag = G * obj1.mass * obj2.mass / dist_sq;

        let dir = vec2(obj2.pos.x - obj1.pos.x, obj2.pos.y - obj1.pos.y).normalize();
        let expected_force = vec2(expected_force_mag * dir.x, expected_force_mag * dir.y);

        let result = force_between(&obj1, &obj2);

        assert!(vec2_approx_eq(result, expected_force, 1e-5));
    }
}
