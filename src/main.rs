extern crate image;

use image::Rgba;

fn main() {
    let mut img = image::open("images/dog.png").unwrap().to_rgba();
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
    img.save("images/output.png").unwrap();
}

#[allow(unused_comparisons)]
fn is_white(r: u8, g: u8, b: u8) -> bool {
    let threshold = 230;
    threshold <= r && r <= 255 && threshold <= g && g <= 255 && threshold <= b && b <= 255
}
