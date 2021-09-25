use crate::algebra_models::Vec3;
use std::io::Write;

struct Pixel {
    x: u8,
    y: u8,
    z: u8,
}

fn float_to_u8(val: f64, max_val: usize) -> u8 {
    (max_val as f64 * val) as u8
}

impl Pixel {
    fn from_vec3(vec3: &Vec3, max_val: usize) -> Self {
        Self {
            x: float_to_u8(vec3.x, max_val),
            y: float_to_u8(vec3.y, max_val),
            z: float_to_u8(vec3.z, max_val),
        }
    }
}

pub fn write_to_ppm(
    file_path: &str,
    width: usize,
    height: usize,
    pixel_buffer: &[Vec3],
    max_val: usize,
) {
    let mut file = std::fs::File::create(file_path).unwrap();
    writeln!(file, "P6").unwrap();
    writeln!(file, "{} {}", width, height).unwrap();
    writeln!(file, "{}", max_val as u8).unwrap();

    for pixel in pixel_buffer {
        let Pixel { x, y, z } = Pixel::from_vec3(pixel, max_val);
        file.write_all(&[x, y, z]).unwrap();
    }
}
