use image::{DynamicImage, Rgba};

pub fn transparent(img: DynamicImage, threshold: u8) -> DynamicImage {
    let mut new_img = img.clone().to_rgba();

    let (width, height) = new_img.dimensions();
    for y in 0..height {
        for x in 0..width {
            let pixel = new_img.get_pixel(x, y);
            let &Rgba([r, g, b, _]) = pixel;
            if is_white(r, g, b, threshold) {
                new_img.put_pixel(x, y, Rgba([r, g, b, 0]));
            }
        }
    }

    DynamicImage::ImageRgba8(new_img)
}

fn is_white(r: u8, g: u8, b: u8, threshold: u8) -> bool {
    threshold <= r && threshold <= g && threshold <= b
}
