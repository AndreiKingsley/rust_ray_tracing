use ray_tracing::geometry::*;
use std::io::Write;
use std::f64::consts::PI;

fn main() {
    const WIDTH: usize = 1024;
    const HEIGHT: usize = 768;
    const FOV: f64 = PI / 3.0;

    let ivory = Material {
        diffuse_color: Vec3 {
            x: 0.4,
            y: 0.4,
            z: 0.3,
        },
        albedo: Vec2 { x: 0.6, y: 0.3 },
        specular_exponent: 50.0,
    };

    let red_rubber = Material {
        diffuse_color: Vec3 {
            x: 0.3,
            y: 0.1,
            z: 0.1,
        },
        albedo: Vec2 { x: 0.9, y: 0.1 },
        specular_exponent: 10.0,
    };

    let mut spheres = vec![];

    spheres.push(
        Sphere {
            center: Vec3 {
                x: -3.0,
                y: 0.0,
                z: -16.0,
            },
            radius: 2.0,
            material: ivory,
        }
    );

    spheres.push(
        Sphere {
            center: Vec3 {
                x: -1.0,
                y: -1.5,
                z: -12.0,
            },
            radius: 2.0,
            material: red_rubber,
        }
    );

    spheres.push(
        Sphere {
            center: Vec3 {
                x: 1.5,
                y: -0.5,
                z: -18.0,
            },
            radius: 3.0,
            material: red_rubber,
        }
    );

    spheres.push(
        Sphere {
            center: Vec3 {
                x: 7.0,
                y: 5.0,
                z: -18.0,
            },
            radius: 4.0,
            material: ivory,
        }
    );

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
                z: -22.0,
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
        }
    ];

    let mut buffer = vec![Vec3 { x: 0.0, y: 0.0, z: 0.0 }; WIDTH * HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = (2.0 * (i as f64 + 0.5) / WIDTH as f64 - 1.0) * (FOV / 2.0).tan() * WIDTH as f64 / HEIGHT as f64;
            let y = -(2.0 * (j as f64 + 0.5) / HEIGHT as f64 - 1.0) * (FOV / 2.0).tan();
            let dir = Vec3 {
                x: x,
                y: y,
                z: -1.0,
            }.normalized();
            buffer[i + j * WIDTH] = cast_ray(
                &Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                &dir,
                &spheres,
                &lights,
            );
        }
    }

    let mut file = std::fs::File::create("file.ppm").unwrap();
    writeln!(file, "P6").unwrap();
    writeln!(file, "{} {}", WIDTH, HEIGHT).unwrap();
    writeln!(file, "{}", 255 as u8).unwrap();

    for i in 0..HEIGHT * WIDTH {
        let cx: u8 = (255.0 * buffer[i].x) as u8;
        let cy: u8 = (255.0 * buffer[i].y) as u8;
        let cz: u8 = (255.0 * buffer[i].z) as u8;

        file.write(&[cx, cy, cz]).unwrap();
    }


    let v1 = Vec3 {
        x: 2.0,
        y: 3.0,
        z: 5.0,
    };
    let v2 = Vec3 {
        x: 7.0,
        y: 11.0,
        z: 13.0,
    };
    println!("{}", v1 * v2);
    println!("{:?}", cross_product(&v1, &v2));
}
