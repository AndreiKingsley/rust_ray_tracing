pub mod geometry {
    use std::ops;

    #[derive(Debug, Copy, Clone)]
    pub struct Vec2 {
        pub x: f64,
        pub y: f64
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl ops::Add<Vec3> for Vec3 {
        type Output = Self;

        fn add(self, rhs: Vec3) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl ops::Sub<Vec3> for Vec3 {
        type Output = Self;

        fn sub(self, rhs: Vec3) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            }
        }
    }

    // Scalar product
    impl ops::Mul<Vec3> for Vec3 {
        type Output = f64;

        fn mul(self, rhs: Vec3) -> Self::Output { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z }
    }

    impl ops::Mul<f64> for Vec3 {
        type Output = Vec3;

        fn mul(self, rhs: f64) -> Self::Output {
            Vec3 {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }

    impl ops::Mul<Vec3> for f64 {
        type Output = Vec3;

        fn mul(self, rhs: Vec3) -> Self::Output {
            Vec3 {
                x: self * rhs.x,
                y: self * rhs.y,
                z: self * rhs.z,
            }
        }
    }

    //Cross product
    pub fn cross_product(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    impl Vec3 {
        pub fn norm(&self) -> f64 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }

        pub fn normalize(&mut self) {
            let norm = self.norm();
            self.x = self.x / norm;
            self.y = self.y / norm;
            self.z = self.z / norm;
        }

        pub fn normalized(&self) -> Vec3 {
            let norm = self.norm();
            Vec3 {
                x: self.x / norm,
                y: self.y / norm,
                z: self.z / norm,
            }
        }
    }

    pub fn reflect(i: &Vec3, n: &Vec3) -> Vec3 {
        *i - *n * 2.0 * (*i * *n)
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Material {
        pub diffuse_color: Vec3,
        pub albedo: Vec2,
        pub specular_exponent: f64
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
        fn ray_intersect(&self, orig: &Vec3, dir: &Vec3, t0: &mut f64) -> bool {
            let l = self.center + *orig;
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

    pub fn scene_intersect(
        orig: &Vec3,
        dir: &Vec3,
        spheres: &Vec<Sphere>,
        hit: &mut Vec3,
        n: &mut Vec3,
        material: &mut Material,
    ) -> bool {
        let mut sphere_dist: f64 = f64::MAX;
        for sphere in spheres {
            let mut dist_i: f64 = 0.0;
            if sphere.ray_intersect(orig, dir, &mut dist_i) && dist_i < sphere_dist {
                sphere_dist = dist_i;
                *hit = *orig + *dir * dist_i;
                *n = (*hit - sphere.center).normalized();
                *material = sphere.material;
            }
        }
        return sphere_dist < 1000.0;
    }

    pub fn cast_ray(
        orig: &Vec3,
        dir: &Vec3,
        spheres: &Vec<Sphere>,
        lights: &Vec<Light>
    ) -> Vec3 {
        let mut point = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut n = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut material = Material {
            diffuse_color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            albedo: Vec2 { x: 0.0, y: 0.0 },
            specular_exponent: 0.0
        };
        if !scene_intersect(orig, dir, &spheres, &mut point, &mut n, &mut material) {
            return Vec3 {
                x: 0.2,
                y: 0.7,
                z: 0.8,
            };
        }

        let mut diffuse_light_intensity = 0.0;
        let mut specular_light_intensity = 0.0;
        for light in lights{
            let light_dir = (light.position - point).normalized();
            let light_coeff = light_dir * n;
            if light_coeff > 0.0 {
                diffuse_light_intensity += light.intensity * light_coeff
            }
            let reflect_coeff = reflect(&light_dir, &n) * *dir;
            if reflect_coeff > 0.0 {
                specular_light_intensity += reflect_coeff.powf(material.specular_exponent) *
                    light.intensity;
            }
        }
        material.diffuse_color * diffuse_light_intensity * material.albedo.x +
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            } * specular_light_intensity * material.albedo.y
    }
}