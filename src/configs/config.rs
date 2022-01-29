extern crate yaml_rust;

use std::env;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};
use yaml_rust::{YamlEmitter, YamlLoader};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub server: pkg::config::Server,
    pub client: pkg::config::Client,
    pub max_threads: usize,
}

impl ApplicationConfig {
    pub fn load_yaml_config(path: String) -> ApplicationConfig {
        let mut file = File::open(path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        let docs = YamlLoader::load_from_str(&contents).expect("Error loading from string");
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&docs[0]).unwrap();
        serde_yaml::from_str(&out_str).expect("Error parsing")
    }
}
