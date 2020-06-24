extern crate yaml_rust;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

pub fn load_config_file() -> (String, String) {
    let default_output = String::from("output");
    let default_threshold = String::from("230");
    if Path::new("touka.yml").exists() {
        let mut f = File::open("touka.yml").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let docs = YamlLoader::load_from_str(&s).unwrap();
        let doc = &docs[0];
        let output = match &doc["output"] {
            Yaml::String(s) => s.to_string(),
            _ => default_output,
        };
        let threshold = match &doc["threshold"] {
            Yaml::Integer(n) => n.to_string(),
            _ => default_threshold,
        };
        (output, threshold)
    } else {
        (default_output, default_threshold)
    }
}
