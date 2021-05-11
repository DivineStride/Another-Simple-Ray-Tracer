use std::f32::consts::PI;

pub mod camera;
pub mod colors;
pub mod controls;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod rays;
pub mod sphere;
pub mod vec3;

const COLOR_CAP: f32 = 256.0;
const CHUNK_SIZE: usize = 16 * 1024;
// const R: f32 = PI / 4.0;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[cfg(test)]
pub mod tests;
