use crossbeam::channel::Sender;
use rand::Rng;
use rayon::prelude::*;
use std::fmt::Write;
use std::io::{Read, Result};

use crate::camera::Camera;
use crate::colors::write_color;
use crate::hittable_list::HittableList;
use crate::rays::RayColor;
use crate::vec3::Vec3 as Color;

use crate::CHUNK_SIZE;

pub fn color_loop(
    camera: &Camera,
    world: &HittableList,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    stats_tx: Sender<usize>,
    render_tx: Sender<Vec<u8>>,
) -> Result<()> {
    // Buffer
    let mut buffer = [0; CHUNK_SIZE];
    let mut file_stream = String::new();
    match write!(file_stream, "P3\n{} {}\n255\n", image_width, image_height) {
        _ => {}
    }

    let samples_per_pixel = if samples_per_pixel > 0 {
        samples_per_pixel
    } else {
        1
    };

    // Render
    for index in 0..=(image_height * image_width) {
        let column = index % image_width;
        let row = image_height - index / image_width;

        let num_read = match file_stream.as_bytes().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        let color: Color = (0..samples_per_pixel)
            .into_par_iter()
            .map(|_sample| {
                let mut rng = rand::thread_rng();

                let u = (column as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                let v = (row as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

                let ray = &camera.get_ray(u, v);
                Color::ray_color(ray, &world, max_depth)
            })
            .sum();

        file_stream.clear();

        match write!(file_stream, "{}", write_color(color, samples_per_pixel)) {
            _ => {}
        }

        let _ = stats_tx.send(num_read);
        if render_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        };
    }

    let _ = stats_tx.send(0);
    let _ = render_tx.send(Vec::new());
    Ok(())
}
