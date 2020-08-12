#[macro_use]
extern crate clap;
extern crate image;

use anyhow::Result;
use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new("touka")
        .about("Making the image background transparent.")
        .version(crate_version!())
        .author("moisutsu moisutsu@gmail.com")
        .arg(Arg::with_name("input_path").help("Input image path."))
        .arg(
            Arg::with_name("output_path")
                .help("Output image path.")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threshold")
                .help("A threshold to determine if a color is white.")
                .short("t")
                .long("threshold")
                .takes_value(true),
        );

    let matches = app.get_matches();

    let mut config = touka::Config::new();
    config.set_cmdline_args(matches);

    let img = touka::load_image(&config).await?;
    let new_img = touka::transparent(img, config.threshold);
    touka::save_image(new_img, &config).await?;

    Ok(())
}
