use anyhow::Result;
use clap::Clap;

use touka::{load_image, save_image, transparent, ImageLocation, Opt};

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();

    let img = load_image(ImageLocation::from(opt.input_path)).await?;
    let transparent_img = transparent(img, opt.threshold);
    save_image(transparent_img, ImageLocation::from(opt.output_path)).await?;

    Ok(())
}
