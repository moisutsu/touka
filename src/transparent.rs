use image::{DynamicImage, Rgba};
use itertools::iproduct;

pub fn transparent(img: DynamicImage, threshold: u8) -> DynamicImage {
    let mut transparent_img = img.to_rgba();

    let (width, height) = transparent_img.dimensions();
    for (y, x) in iproduct!(0..height, 0..width) {
        let pixel = transparent_img.get_pixel(x, y);
        let &Rgba([r, g, b, _]) = pixel;
        if is_white(r, g, b, threshold) {
            transparent_img.put_pixel(x, y, Rgba([r, g, b, 0]));
        }
    }

    DynamicImage::ImageRgba8(transparent_img)
}

fn is_white(r: u8, g: u8, b: u8, threshold: u8) -> bool {
    threshold <= r && threshold <= g && threshold <= b
}
