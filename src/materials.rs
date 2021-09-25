use crate::algebra_models::*;
use crate::physical_models::*;

pub const IVORY: Material = Material {
    diffuse_color: Vec3 {
        x: 0.4,
        y: 0.4,
        z: 0.3,
    },
    albedo: Vec4 {
        x: 0.6,
        y: 0.3,
        z: 0.1,
        w: 0.0,
    },
    specular_exponent: 50.0,
    refractive_index: 1.0,
};

pub const RED_RUBBER: Material = Material {
    diffuse_color: Vec3 {
        x: 0.3,
        y: 0.1,
        z: 0.1,
    },
    albedo: Vec4 {
        x: 0.9,
        y: 0.1,
        z: 0.0,
        w: 0.0,
    },
    specular_exponent: 10.0,
    refractive_index: 1.0,
};

pub const MIRROR: Material = Material {
    diffuse_color: Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
    albedo: Vec4 {
        x: 0.0,
        y: 10.0,
        z: 0.8,
        w: 0.0,
    },
    specular_exponent: 1425.0,
    refractive_index: 1.0,
};

pub const GLASS: Material = Material {
    diffuse_color: Vec3 {
        x: 0.6,
        y: 0.7,
        z: 0.8,
    },
    albedo: Vec4 {
        x: 0.0,
        y: 0.5,
        z: 0.1,
        w: 0.8,
    },
    specular_exponent: 125.0,
    refractive_index: 1.5,
};
