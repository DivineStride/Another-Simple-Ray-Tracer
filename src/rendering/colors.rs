use crate::vec3::Color;
use crate::COLOR_CAP;

pub fn write_color(color: Color, samples_per_pixel: u32) -> String {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f32;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    format!(
        "{} {} {}\n",
        (COLOR_CAP * r.clamp(0.0, 0.9999)) as u32,
        (COLOR_CAP * g.clamp(0.0, 0.9999)) as u32,
        (COLOR_CAP * b.clamp(0.0, 0.9999)) as u32
    )
}
