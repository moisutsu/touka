use anyhow::Result;

use crate::Config;
use crate::ImageLocation;
use image::DynamicImage;

pub async fn load_image(config: &Config) -> Result<DynamicImage> {
    let img = if let ImageLocation::File(input_path) = &config.input_path {
        image::open(input_path)?
    } else {
        imboard::copy_image::from_clipboard().await?
    };
    Ok(img)
}

pub async fn save_image(img: DynamicImage, config: &Config) -> Result<()> {
    match &config.output_path {
        ImageLocation::File(output_path) => img.save(format!("{}.png", output_path))?,
        ImageLocation::Clipboard => imboard::copy_image::to_clipboard(img).await?,
    }
    Ok(())
}
