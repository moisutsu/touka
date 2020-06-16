extern crate image;

fn main() {
    let img = image::open("images/dog.png").unwrap().to_rgb();
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            println!("{:?}", pixel);
        }
    }
}
