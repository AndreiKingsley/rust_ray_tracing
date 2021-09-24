pub mod geometry {
    use std::ops;

    #[derive(Debug, Copy, Clone)]
    pub struct Vec4 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }

    #[derive(Debug, Copy, Clone, Default)]
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

    pub fn refract(i: &Vec3, n: &Vec3, eta_t: f64, eta_i: f64) -> Vec3 {
        let cosi = (-1.0) * (*i * *n).clamp(-1.0, 1.0);
        if cosi < 0.0 {
            return refract(i, &(*n * -1.0), eta_i, eta_t);
        }
        let eta = eta_i / eta_t;
        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        if k < 0.0 {
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            *i * eta + *n * (eta * cosi - k.sqrt())
        }
    }

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
        fn ray_intersect(&self, orig: &Vec3, dir: &Vec3, t0: &mut f64) -> bool {
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

    pub fn scene_intersect(
        orig: &Vec3,
        dir: &Vec3,
        spheres: &Vec<Sphere>,
        hit: &mut Vec3,
        n: &mut Vec3,
        material: &mut Material,
    ) -> bool {
        let mut spheres_dist: f64 = f64::MAX;
        for sphere in spheres {
            let mut dist_i: f64 = 0.0;
            if sphere.ray_intersect(orig, dir, &mut dist_i) && dist_i < spheres_dist {
                spheres_dist = dist_i;
                *hit = *orig + *dir * dist_i;
                *n = (*hit - sphere.center).normalized();
                *material = sphere.material;
            }
        }

        let mut checkerboard_dist = f64::MAX;
        if dir.y.abs() > 1e-3 {
            let d = -1.0 * (orig.y + 4.0) / dir.y;
            let pt = *orig + *dir * d;
            if d > 0.0 && pt.x.abs() < 10.0 && pt.z < -10.0 && pt.z > -30.0 && d < spheres_dist {
                checkerboard_dist = d;
                *hit = pt;
                *n = Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                };
                material.diffuse_color =
                    if (((0.5 * hit.x + 1000.0) as i32) + ((0.5 * hit.z) as i32)) & 1 == 1 {
                        Vec3 {
                            x: 0.3,
                            y: 0.3,
                            z: 0.3,
                        }
                    } else {
                        Vec3 {
                            x: 0.3,
                            y: 0.2,
                            z: 0.1,
                        }
                    }
            }
        }
        return spheres_dist.min(checkerboard_dist) < 1000.0;
    }

    pub fn cast_ray(
        orig: &Vec3,
        dir: &Vec3,
        spheres: &Vec<Sphere>,
        lights: &Vec<Light>,
        depth: usize,
    ) -> Vec3 {
        let mut point = Vec3::default();
        let mut n = Vec3::default();
        let mut material = Material::default();
        if depth > 4 || !scene_intersect(orig, dir, &spheres, &mut point, &mut n, &mut material) {
            return Vec3 {
                x: 0.2,
                y: 0.7,
                z: 0.8,
            };
        }

        let reflect_dir = reflect(dir, &n).normalized();
        let reflect_orig = if reflect_dir * n < 0.0 {
            point - n * 1e-3
        } else {
            point + n * 1e-3
        };

        let reflect_color = cast_ray(&reflect_orig, &reflect_dir, spheres, lights, depth + 1);

        let refract_dir = refract(dir, &n, material.refractive_index, 1.0).normalized();
        let refract_orig = if refract_dir * n < 0.0 {
            point - n * 1e-3
        } else {
            point + n * 1e-3
        };

        let refract_color = cast_ray(&refract_orig, &refract_dir, spheres, lights, depth + 1);

        let mut diffuse_light_intensity = 0.0;
        let mut specular_light_intensity = 0.0;
        for light in lights {
            let light_dir = (light.position - point).normalized();
            let light_distance = (light.position - point).norm();

            let shadow_orig = if light_dir * n < 0.0 {
                point - n * 1e-3
            } else {
                point + n * 1e-3
            };

            let mut shadow_pt = Vec3::default();
            let mut shadow_n = Vec3::default();

            let mut tmp_material = Material::default();

            if scene_intersect(
                &shadow_orig,
                &light_dir,
                spheres,
                &mut shadow_pt,
                &mut shadow_n,
                &mut tmp_material,
            ) && (shadow_pt - shadow_orig).norm() < (light.position - point).norm() {
                continue;
            }


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
                z: 1.0,
            } *
                specular_light_intensity * material.albedo.y
            + reflect_color * material.albedo.z + refract_color * material.albedo.w
    }
}
