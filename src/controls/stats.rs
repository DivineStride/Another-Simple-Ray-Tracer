pub mod timer;

use crossbeam::channel::Receiver;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};
use std::io::{stderr, Result, Stderr, Write};
use std::time::Instant;
use timer::Timer;

pub fn stats_loop(stats_rx: Receiver<usize>, expected: usize) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();

    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rates_per_second = num_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += num_bytes;
        if timer.ready {
            timer.ready = false;
            output_progress(
                &mut stderr(),
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rates_per_second,
                expected,
            );
        }
        if num_bytes == 0 {
            break;
        }
    }
    Ok(())
}

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64, expected: usize) {
    let bytes_styled = style::style(format!("{:0<6}/{} ", bytes, expected)).with(Color::Yellow);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(
        " [{:.0}b/s ETR: {}]",
        rate,
        (expected as u64 - bytes as u64 / rate as u64).as_time()
    ))
    .with(Color::Cyan);
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes_styled),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );
    let _ = stderr.flush();
}

pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    /// Renders the u64 into a time string
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}
