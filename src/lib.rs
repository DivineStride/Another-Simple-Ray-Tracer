use std::f32::consts::PI;

pub mod controls;
pub mod materials;
pub mod rendering;
pub mod shapes;
pub mod threading;
pub mod vec3;

const COLOR_CAP: f32 = 256.0;
const CHUNK_SIZE: usize = 16 * 1024;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[cfg(test)]
pub mod tests;
