use clap::ArgMatches;

pub struct Config {
    pub input_path: String,
    pub output_path: String,
    pub threshold: u8,
}

impl Config {
    pub fn new() -> Config {
        // default config value
        Config {
            input_path: "default".to_string(),
            output_path: "output".to_string(),
            threshold: 230,
        }
    }

    pub fn set_cmdline_args(&mut self, matches: ArgMatches) {
        self.input_path = matches.value_of("input_path").unwrap().to_string();
        if let Some(output_path) = matches.value_of("output_path") {
            self.output_path = output_path.to_string();
        }
        if let Some(threshold) = matches.value_of("threshold") {
            self.threshold = threshold.parse().unwrap();
        }
    }
}
