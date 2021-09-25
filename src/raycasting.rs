use crate::algebra_models::*;
use crate::physical_models::*;

const EPS: f64 = 1e-3;

const BACKGROUND_COLOR: Vec3 = Vec3 {
    x: 0.2,
    y: 0.7,
    z: 0.8,
};

const CHECKERBOARD_COLOR_1: Vec3 = Vec3 {
    x: 0.3,
    y: 0.3,
    z: 0.3,
};

const CHECKERBOARD_COLOR_2: Vec3 = Vec3 {
    x: 0.3,
    y: 0.2,
    z: 0.1,
};

fn get_orig(point: &Vec3, dir: &Vec3, normal: &Vec3) -> Vec3 {
    if *dir * *normal < 0.0 {
        *point - *normal * EPS
    } else {
        *point + *normal * EPS
    }
}

fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
    *vec - *normal * 2.0 * (*vec * *normal)
}

fn refract(vec: &Vec3, normal: &Vec3, eta_t: f64, eta_i: f64) -> Vec3 {
    let cosi = (-1.0) * (*vec * *normal).clamp(-1.0, 1.0);
    if cosi < 0.0 {
        return refract(vec, &(*normal * -1.0), eta_i, eta_t);
    }
    let eta = eta_i / eta_t;
    let refract_cos2 = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if refract_cos2 < 0.0 {
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    } else {
        *vec * eta + *normal * (eta * cosi - refract_cos2.sqrt())
    }
}

fn scene_intersect(
    orig: &Vec3,
    dir: &Vec3,
    spheres: &[Sphere],
    hit: &mut Vec3,
    normal: &mut Vec3,
    material: &mut Material,
) -> bool {
    let mut spheres_dist: f64 = f64::MAX;
    for sphere in spheres {
        let mut dist_i: f64 = 0.0;
        if sphere.ray_intersect(orig, dir, &mut dist_i) && dist_i < spheres_dist {
            spheres_dist = dist_i;
            *hit = *orig + *dir * dist_i;
            *normal = (*hit - sphere.center).normalized();
            *material = sphere.material;
        }
    }

    let mut checkerboard_dist = f64::MAX;
    if dir.y.abs() > EPS {
        let d = -1.0 * (orig.y + 4.0) / dir.y;
        let pt = *orig + *dir * d;
        if d > 0.0 && pt.x.abs() < 10.0 && pt.z < -10.0 && pt.z > -30.0 && d < spheres_dist {
            checkerboard_dist = d;
            *hit = pt;
            *normal = Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            };
            material.diffuse_color =
                if (((0.5 * hit.x + 1000.0) as i32) + ((0.5 * hit.z) as i32)) & 1 == 1 {
                    CHECKERBOARD_COLOR_1
                } else {
                    CHECKERBOARD_COLOR_2
                }
        }
    }
    spheres_dist.min(checkerboard_dist) < 1000.0
}

pub fn cast_ray(
    orig: &Vec3,
    dir: &Vec3,
    spheres: &[Sphere],
    lights: &[Light],
    depth: usize,
) -> Vec3 {
    let mut point = Vec3::default();
    let mut normal = Vec3::default();
    let mut material = Material::default();
    if depth > 4 || !scene_intersect(orig, dir, spheres, &mut point, &mut normal, &mut material) {
        return BACKGROUND_COLOR;
    }

    let reflect_dir = reflect(dir, &normal).normalized();
    let reflect_orig = get_orig(&point, &reflect_dir, &normal);
    let reflect_color = cast_ray(&reflect_orig, &reflect_dir, spheres, lights, depth + 1);

    let refract_dir = refract(dir, &normal, material.refractive_index, 1.0).normalized();
    let refract_orig = get_orig(&point, &refract_dir, &normal);
    let refract_color = cast_ray(&refract_orig, &refract_dir, spheres, lights, depth + 1);

    let mut diffuse_light_intensity = 0.0;
    let mut specular_light_intensity = 0.0;
    for light in lights {
        let light_dir = (light.position - point).normalized();
        let shadow_orig = get_orig(&point, &light_dir, &normal);

        let mut shadow_pt = Vec3::default();
        let mut shadow_normal = Vec3::default();

        let mut tmp_material = Material::default();

        if scene_intersect(
            &shadow_orig,
            &light_dir,
            spheres,
            &mut shadow_pt,
            &mut shadow_normal,
            &mut tmp_material,
        ) && (shadow_pt - shadow_orig).norm() < (light.position - point).norm()
        {
            continue;
        }

        let intensity_coefficient = light_dir * normal;
        if intensity_coefficient > 0.0 {
            diffuse_light_intensity += light.intensity * intensity_coefficient
        }

        let reflect_coefficient = reflect(&light_dir, &normal) * *dir;
        if reflect_coefficient > 0.0 {
            specular_light_intensity +=
                reflect_coefficient.powf(material.specular_exponent) * light.intensity;
        }
    }
    material.diffuse_color * diffuse_light_intensity * material.albedo.x
        + Vec3::ones() * specular_light_intensity * material.albedo.y
        + reflect_color * material.albedo.z
        + refract_color * material.albedo.w
}
