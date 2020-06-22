use image::{ImageBuffer, Rgba};

pub fn transparent(
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    threshold: u8,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut new_img = img.clone();
    let (width, height) = img.dimensions();
    for (x, y) in (0..width).zip(0..height) {
        let pixel = img.get_pixel(x, y);
        let &Rgba([r, g, b, _]) = pixel;
        if is_white(r, g, b, threshold) {
            new_img.put_pixel(x, y, Rgba([r, g, b, 0]));
        }
    }
    new_img
}

fn is_white(r: u8, g: u8, b: u8, threshold: u8) -> bool {
    threshold <= r && threshold <= g && threshold <= b
}
