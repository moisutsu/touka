use anyhow::Result;

use crate::ImageLocation;
use image::DynamicImage;

pub async fn load_image(location: ImageLocation) -> Result<DynamicImage> {
    let img = if let ImageLocation::File(input_path) = location {
        image::open(input_path)?
    } else {
        imboard::copy_image::from_clipboard().await?
    };
    Ok(img)
}

pub async fn save_image(img: DynamicImage, location: ImageLocation) -> Result<()> {
    match location {
        ImageLocation::File(output_path) => img.save(output_path.with_extension("png"))?,
        ImageLocation::Clipboard => imboard::copy_image::to_clipboard(img).await?,
    }
    Ok(())
}
