#[macro_use]
extern crate clap;
extern crate image;

use clap::{App, Arg};
use image::Rgba;

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
    let input_path = matches.value_of("input_path").unwrap();
    let output_path = matches.value_of("output_path").unwrap_or("output");
    let threshold = matches
        .value_of("threshold")
        .unwrap_or("230")
        .parse()
        .unwrap();

    let mut img = image::open(&input_path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    for (x, y) in (0..width).zip(0..height) {
        let pixel = img.get_pixel(x, y);
        let &Rgba([r, g, b, _]) = pixel;
        if is_white(r, g, b, threshold) {
            img.put_pixel(x, y, Rgba([r, g, b, 0]));
        }
    }
    img.save(format!("{}.png", output_path)).unwrap();
}

fn is_white(r: u8, g: u8, b: u8, threshold: u8) -> bool {
    threshold <= r && threshold <= g && threshold <= b
}
