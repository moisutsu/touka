extern crate clap;
extern crate image;

use clap::{App, Arg};
use image::Rgba;

fn main() {
    let app = App::new("touka")
        .about("Making the image background transparent.")
        .arg(
            Arg::with_name("input_path")
                .help("Input image path.")
                .required(true),
        )
        .arg(
            Arg::with_name("output_path")
                .help("Output image path.")
                .short("o")
                .takes_value(true),
        );

    let matches = app.get_matches();
    let input_path = matches.value_of("input_path").unwrap();
    let output_path = matches.value_of("output_path").unwrap_or("output");

    let mut img = image::open(&input_path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            if is_white(r, g, b) {
                img.put_pixel(x, y, Rgba([r, g, b, 0]));
            }
        }
    }
    img.save(format!("{}.png", output_path)).unwrap();
}

#[allow(unused_comparisons)]
fn is_white(r: u8, g: u8, b: u8) -> bool {
    let threshold = 230;
    threshold <= r && r <= 255 && threshold <= g && g <= 255 && threshold <= b && b <= 255
}
