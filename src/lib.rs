// Copyright (c) Fabian Beskow 2024

mod defaults;
mod config;
mod post;
mod index;

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
    let entries = fs::read_dir(dir).or(Err(format!("Cannot find \"{}\"", dir)))?;

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
#[derive(Debug)]
pub struct Website {
    config: Config,
    posts: Vec<Post>,
}

impl Website {

    /// Initialize a project
    pub fn init(path: &str) -> Result<(), Box<dyn Error>> {
        if !check_if_dir_empty(path)? { // Avoid overwriting existing projects/dirs
            eprintln!("Directory {} not empty!", path);
            return Err("Directory not empty!".into());
        }

        env::set_current_dir(Path::new(path))?;

        // Init the directory structure and content
        defaults::create_default_dirs()?;
        defaults::create_default_files()?;
        
        Ok(())
    }

    /// Generate the output for a project
    pub fn gen(path: &str) -> Result<(), Box<dyn Error>> {
        env::set_current_dir(Path::new(path))?;
        let website = Self::from_working()?;
        
        eprintln!("{:?}", website);

        for post in website.posts {
            post.output_to_file(&website.config, defaults::OUT_DIR)?;
        }

        Ok(())
    }

    /// Creates the struct from files in the working directory
    fn from_working() -> Result<Website, Box<dyn Error>>{
        //let mut entries = fs::read_dir(directory)?;
        Ok(Website {
            config: Config::from_file(defaults::CONFIG_FILE)?,
            posts: rlist_files(defaults::CONTENT_DIR)?
                .iter()
                //.map(|file| {println!("{}", file); return file;})
                .map(|file| Post::from_file(file).unwrap_or_default())
                .collect(),
        })
    }

}


