extern crate image;

use image::GenericImageView;

fn main() {
    let img = image::open("images/dog.png").unwrap();
    let (width, height) = img.dimensions();
    println!("{} {}", width, height);
}
