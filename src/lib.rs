// Copyright (c) Fabian Beskow 2024

mod post;

use crate::post::Post;

use std::fs;
use std::error::Error;
//use std::path::Path;

/// Check if a dir is empty to avoid overwriting projects etc.
fn check_if_dir_empty(directory: &str) -> Result<bool, std::io::Error> {
    let mut entries = fs::read_dir(directory)?;
    let first_entry = entries.next();
    Ok(first_entry.is_none())
}

/// Any configuration that covers the whole project is fit in here
pub struct Config {
    name: String,
    base_path: String,
    domain: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {

        Ok(Config {
            name: "unimplemented".into(),
            base_path: "/unimplemented/".into(),
            domain: "example.com".into(),
        })
    }
}

/// The top struct of a project
pub struct Website {
    config: Config,
    posts: Vec<Post>,
}

impl Website {
    /// Initialize a project
    pub fn init(path: &str) -> Result<(), Box<dyn Error>> {
        check_if_dir_empty(path)?;
        todo!();
        Ok(())
    }

    /// Generate the output for a project
    pub fn gen(path: &str) -> Result<(), Box<dyn Error>> {
        todo!();
        Ok(())
    }
}


