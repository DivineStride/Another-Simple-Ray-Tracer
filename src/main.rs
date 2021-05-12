use crossbeam::channel::{bounded, unbounded};
use std::io::Result;
use std::thread;

use ray_tracer::camera::Camera;
use ray_tracer::controls::args::Args;
use ray_tracer::controls::world::get_world;
use ray_tracer::controls::{colors, render, stats};
use ray_tracer::vec3::{Vec3, Vec3 as Point3};

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        aspect_ratio,
        image_width,
        image_height,
        samples_per_pixel,
        outfile,
        max_depth,
    } = args;

    let (stats_tx, stats_rx) = unbounded();
    let (render_tx, render_rx) = bounded(1024);

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);

    // Camera
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    // World
    let world = get_world(true);

    let color_handle = thread::spawn(move || {
        colors::color_loop(
            &camera,
            &world,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            stats_tx,
            render_tx,
        )
    });

    let stats_handle = thread::spawn(move || {
        stats::stats_loop(
            stats_rx,
            ((image_width as f32 * image_height as f32 * 11.3) as usize + 24) as usize,
        )
    });
    let render_handle = thread::spawn(move || render::render_loop(&outfile, render_rx));

    let color_io_result = color_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let render_io_result = render_handle.join().unwrap();

    color_io_result?;
    stats_io_result?;
    render_io_result?;

    Ok(())
}
