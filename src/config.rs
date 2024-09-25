// Copyright (c) Fabian Beskow 2024

use std::fs;
use std::error::Error;
use serde_derive::Deserialize;

/// Any configuration that covers the whole project is fit in here
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub blog_name: String,
    pub base_path: String,
    pub domain: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            blog_name: "unimplemented".into(),
            base_path: "/blog/".into(),
            domain: "blog.example.com".into(),
        }
    }
}

impl Config {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let config: Config = toml::from_str(s)?;
        Ok(config)
    }

    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let config_string = fs::read_to_string(file_path)?;
        Self::from_str(&config_string)
    }
}

