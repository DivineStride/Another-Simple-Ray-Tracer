use clap::{App, Arg};

pub struct Args {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub outfile: String,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("ray_tracer")
            .arg(
                Arg::new("aspect_ratio")
                    .short('a')
                    .long("aspect")
                    .takes_value(true)
                    .about("Default 16 / 9: 1.7778"),
            )
            .arg(
                Arg::new("image_width")
                    .short('w')
                    .long("width")
                    .takes_value(true)
                    .about("Default 400: height * aspect ratio"),
            )
            .arg(
                Arg::new("image_height")
                    .short('h')
                    .long("height")
                    .takes_value(true)
                    .about("Default 225: width / aspect ratio"),
            )
            .arg(
                Arg::new("samples_per_pixel")
                    .short('s')
                    .long("samples")
                    .takes_value(true)
                    .about("Default 100"),
            )
            .arg(
                Arg::new("max_depth")
                    .short('d')
                    .long("depth")
                    .takes_value(true)
                    .about("Default 50"),
            )
            .arg(
                Arg::new("outfile")
                    .short('o')
                    .long("outfile")
                    .takes_value(true)
                    .about("Write output to a file instead of stdout"),
            )
            .get_matches();

        let (default_width, default_height, default_aspect, default_samples, default_depth) =
            (400, 225, 1.7778, 100, 50);

        let mut aspect_ratio = matches
            .value_of("aspect_ratio")
            .unwrap_or(&format!("{}", default_aspect)[..])
            .parse::<f32>()
            .unwrap();
        let mut image_height = matches
            .value_of("image_height")
            .unwrap_or(&format!("{}", default_height)[..])
            .parse::<u32>()
            .unwrap();
        let mut image_width = matches
            .value_of("image_width")
            .unwrap_or(&format!("{}", default_width)[..])
            .parse::<u32>()
            .unwrap();
        let samples_per_pixel = matches
            .value_of("samples_per_pixel")
            .unwrap_or(&format!("{}", default_samples)[..])
            .parse::<u32>()
            .unwrap();
        let max_depth = matches
            .value_of("max_depth")
            .unwrap_or(&format!("{}", default_depth)[..])
            .parse::<u32>()
            .unwrap();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();

        let aspect_check_1 = aspect_ratio - default_aspect;
        let aspect_check_2 = aspect_ratio - (image_width / image_height) as f32;

        if aspect_check_1 < 0.01 || aspect_check_2 < 0.01 {
            aspect_ratio = image_width as f32 / image_height as f32;

            if image_width != default_width {
                image_width = (image_height as f32 * aspect_ratio) as u32;
            }

            if image_height != default_height {
                image_height = (image_width as f32 / aspect_ratio) as u32;
            }
        }

        Self {
            image_height,
            image_width,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            outfile,
        }
    }
}
