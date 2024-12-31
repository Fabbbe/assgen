// Copyright (c) Fabian Beskow 2024

use crate::index::Index;

use std::fs;
use std::error::Error;
use serde_derive::Deserialize;


/// Any index should have at least a path and a title
#[derive(Debug, PartialEq, Deserialize)]
pub struct IndexConfig {
    pub path: String,
    pub title: String,
}

/// Any configuration that covers the whole project is fit in here
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub blog_name: String,
    pub base_path: String,
    pub domain: String,
    pub indexes: Vec<IndexConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            blog_name: "unimplemented".into(),
            base_path: "".into(),
            domain: "blog.example.com".into(),
            indexes: vec![IndexConfig {path: "/".into(), title: "Index of /".into()}],
        }
    }
}

impl Config {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        // TODO: change to or_default
        let config: Config = toml::from_str(s)?;
        Ok(config)
    }

    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let config_string = fs::read_to_string(file_path)?;
        Self::from_str(&config_string)
    }
}

