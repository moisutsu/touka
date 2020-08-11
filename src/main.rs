#[macro_use]
extern crate clap;
extern crate image;

use clap::{App, Arg};

fn main() {
    let app = App::new("touka")
        .about("Making the image background transparent.")
        .version(crate_version!())
        .author("moisutsu moisutsu@gmail.com")
        .arg(
            Arg::with_name("input_path")
                .help("Input image path.")
                .required(true),
        )
        .arg(
            Arg::with_name("output_path")
                .help("Output image path.")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threshold")
                .help("A threshold to determine if a color is white.")
                .short("t")
                .long("threshold")
                .takes_value(true),
        );

    let matches = app.get_matches();

    let mut config = touka::Config::new();
    config.set_cmdline_args(matches);

    let img = image::open(&config.input_path).unwrap();
    let new_img = touka::transparent(img, config.threshold);
    new_img.save(format!("{}.png", config.output_path)).unwrap();
}
