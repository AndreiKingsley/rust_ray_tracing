use crate::algebra_models::*;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub diffuse_color: Vec3,
    pub albedo: Vec4,
    pub specular_exponent: f64,
    pub refractive_index: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            diffuse_color: Default::default(),
            albedo: Vec4 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            specular_exponent: 0.0,
            refractive_index: 1.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
}

impl Sphere {
    pub(crate) fn ray_intersect(&self, orig: &Vec3, dir: &Vec3, t0: &mut f64) -> bool {
        let l = self.center - *orig;
        let tca = l * (*dir);
        let d2 = l * l - tca * tca;
        let radius_squared = self.radius * self.radius;
        if d2 > radius_squared {
            return false;
        }
        let thc = (radius_squared - d2).sqrt();
        *t0 = tca - thc;
        let t1 = tca + thc;
        if *t0 < 0.0 {
            *t0 = t1;
        }
        if *t0 < 0.0 {
            return false;
        }
        true
    }
}
