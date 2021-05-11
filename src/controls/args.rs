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

        let default_width = 400;
        let default_height = 225;
        let default_aspect = 1.777778;
        let default_samples = 100;
        let default_depth = 50;

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

        let ratio_check = if image_height > image_width {
            (image_height / image_width) as f32
        } else {
            (image_width / image_height) as f32
        };

        if !((aspect_ratio - (ratio_check) as f32).abs() <= f32::EPSILON) {
            if image_height > image_width {
                image_width = (image_height as f32 / aspect_ratio) as u32;
                eprintln!("{:?}", image_width);
            } else {
                image_height = (image_width as f32 / aspect_ratio) as u32;
                eprintln!("{:?}", image_height);
            }

            if !((aspect_ratio - default_aspect) <= f32::EPSILON) {
                aspect_ratio = if image_height > image_width {
                    (image_height / image_width) as f32
                } else {
                    (image_width / image_height) as f32
                }
            }
            eprintln!("{:?}", aspect_ratio);
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
