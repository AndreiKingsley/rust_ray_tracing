use ray_tracing::algebra_models::*;
use ray_tracing::materials::*;
use ray_tracing::physical_models::*;
use ray_tracing::ppm::write_to_ppm;
use ray_tracing::raycasting::cast_ray;
use std::f64::consts::PI;

fn main() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;
    const FOV: f64 = PI / 3.0;

    let spheres = vec![
        Sphere {
            center: Vec3 {
                x: -3.0,
                y: 0.0,
                z: -16.0,
            },
            radius: 2.0,
            material: IVORY,
        },
        Sphere {
            center: Vec3 {
                x: -1.0,
                y: -1.5,
                z: -12.0,
            },
            radius: 2.0,
            material: GLASS,
        },
        Sphere {
            center: Vec3 {
                x: 1.5,
                y: -0.5,
                z: -18.0,
            },
            radius: 3.0,
            material: RED_RUBBER,
        },
        Sphere {
            center: Vec3 {
                x: 7.0,
                y: 5.0,
                z: -18.0,
            },
            radius: 4.0,
            material: MIRROR,
        },
    ];

    let lights = vec![
        Light {
            position: Vec3 {
                x: -20.0,
                y: 20.0,
                z: 20.0,
            },
            intensity: 1.5,
        },
        Light {
            position: Vec3 {
                x: 30.0,
                y: 50.0,
                z: -25.0,
            },
            intensity: 1.8,
        },
        Light {
            position: Vec3 {
                x: 30.0,
                y: 20.0,
                z: 30.0,
            },
            intensity: 1.7,
        },
    ];

    let mut pixel_buffer = vec![Vec3::default(); WIDTH * HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x =
                (2.0 * (i as f64 + 0.5) / WIDTH as f64 - 1.0) * (FOV / 2.0).tan() * WIDTH as f64
                    / HEIGHT as f64;
            let y = -(2.0 * (j as f64 + 0.5) / HEIGHT as f64 - 1.0) * (FOV / 2.0).tan();
            let dir = Vec3 { x, y, z: -1.0 }.normalized();
            pixel_buffer[i + j * WIDTH] = cast_ray(&Vec3::default(), &dir, &spheres, &lights, 0);
        }
    }

    write_to_ppm("image.ppm", WIDTH, HEIGHT, &*pixel_buffer, 255);
}
