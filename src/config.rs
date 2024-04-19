// src/config.rs
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

use crate::provider::Provider;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: Provider,
    pub model_name: Option<String>,
    pub region: Option<String>,
    pub profile: Option<String>,
    pub max_token: u32,
    pub temperature: f32,
    pub endpoint: Option<String>,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}