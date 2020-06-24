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

    let (output, threshold) = touka::load_config_file();

    let matches = app.get_matches();
    let input_path = matches.value_of("input_path").unwrap();
    let output_path = matches.value_of("output_path").unwrap_or(&output);
    let threshold = matches
        .value_of("threshold")
        .unwrap_or(&threshold)
        .parse()
        .unwrap();

    let img = image::open(&input_path).unwrap().to_rgba();
    let new_img = touka::transparent(img, threshold);
    new_img.save(format!("{}.png", output_path)).unwrap();
}
