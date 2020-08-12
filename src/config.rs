use crate::ImageLocation;
use clap::ArgMatches;

pub struct Config {
    pub input_path: ImageLocation,
    pub output_path: ImageLocation,
    pub threshold: u8,
}

impl Config {
    pub fn new() -> Config {
        // default config value
        Config {
            input_path: ImageLocation::Clipboard,
            output_path: ImageLocation::Clipboard,
            threshold: 230,
        }
    }

    pub fn set_cmdline_args(&mut self, matches: ArgMatches) {
        if let Some(input_path) = matches.value_of("input_path") {
            self.input_path = ImageLocation::File(input_path.to_string());
        }
        if let Some(output_path) = matches.value_of("output_path") {
            self.output_path = ImageLocation::File(output_path.to_string());
        }
        if let Some(threshold) = matches.value_of("threshold") {
            self.threshold = threshold.parse().unwrap();
        }
    }
}
