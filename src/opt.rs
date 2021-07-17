use clap::{crate_authors, crate_description, crate_name, crate_version, Clap};
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), name=crate_name!(), about=crate_description!())]
pub struct Opt {
    #[clap(parse(from_os_str), about = "Input image path")]
    pub input_path: Option<PathBuf>,
    #[clap(short, long, parse(from_os_str), about = "Output image path")]
    pub output_path: Option<PathBuf>,
    #[clap(
        short,
        long,
        default_value = "230",
        about = "A threshold to determine if a color is white"
    )]
    pub threshold: u8,
}
