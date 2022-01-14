use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Opt {
    #[clap(parse(from_os_str), help = "Input image path")]
    pub input_path: Option<PathBuf>,
    #[clap(short, long, parse(from_os_str), help = "Output image path")]
    pub output_path: Option<PathBuf>,
    #[clap(
        short,
        long,
        default_value = "230",
        help = "A threshold to determine if a color is white"
    )]
    pub threshold: u8,
}
