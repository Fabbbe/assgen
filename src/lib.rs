// Copyright (c) Fabian Beskow 2024

mod post;
mod config;
mod defaults;

use crate::post::Post;
use crate::config::Config;

use std::fs;
use std::env;
use std::error::Error;
use std::path::Path;

/// Check if a dir is empty to avoid overwriting projects etc.
fn check_if_dir_empty(directory: &str) -> Result<bool, std::io::Error> {
    let mut entries = fs::read_dir(directory)?;
    let first_entry = entries.next();
    Ok(first_entry.is_none())
}

/// Recursivly list files in directory this might break if recursivly symlinked
fn rlist_files(dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files: Vec<String> = Vec::new();
    let entries = fs::read_dir(dir)?;

    for entry in entries { 
        let entry = entry?;
        if entry.file_type()?.is_dir() { // Recurse into next dir
            if let Some(dir) = entry.path().to_str() { 
                if let Ok(mut dir_files) = rlist_files(dir) {
                    files.append(dir_files.as_mut());
                }
            }
        } else if let Some(file) = entry.path().to_str() { // Append file
            files.push(file.to_string())
        } 
        
    }
    Ok(files)
}

/// The top struct of a project
pub struct Website {
    config: Config,
    posts: Vec<Post>,
}

impl Website {
    /// Creates the struct from files in the working directory
    fn from_working() -> Result<Website, Box<dyn Error>>{
        let posts
        let mut entries = fs::read_dir(directory)?;
        Ok(Website {
            config: Config::from_file(defaults::CONFIG_FILE)?,
            posts: 
        })
    }

    /// Initialize a project
    pub fn init(path: &str) -> Result<(), Box<dyn Error>> {
        check_if_dir_empty(path)?;
        todo!();
        Ok(())
    }

    /// Generate the output for a project
    pub fn gen(path: &str) -> Result<(), Box<dyn Error>> {
        env::set_current_dir(Path::new(path))?;

        todo!();
        Ok(())
    }
}


