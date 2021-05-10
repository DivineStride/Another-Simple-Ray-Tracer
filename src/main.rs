use crossbeam::channel::{bounded, unbounded};
use std::io::Result;
use std::thread;

use ray_tracer::controls::args::Args;
use ray_tracer::controls::{colors, render, stats};

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

    let color_handle = thread::spawn(move || {
        colors::color_loop(
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            stats_tx,
            render_tx,
        )
    });

    let stats_handle = thread::spawn(move || stats::stats_loop(stats_rx));
    let render_handle = thread::spawn(move || render::render_loop(&outfile, render_rx));

    let color_io_result = color_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let render_io_result = render_handle.join().unwrap();

    color_io_result?;
    stats_io_result?;
    render_io_result?;

    Ok(())
}
