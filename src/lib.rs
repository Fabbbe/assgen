// Copyright (c) Fabian Beskow 2024

mod defaults;
mod config;
mod post;
mod utils;

use crate::post::Post;
use crate::config::Config;

use std::env;
use std::error::Error;
use std::path::Path;



/// The top struct of a project
#[derive(Debug)]
pub struct Website {
    config: Config,
    posts: Vec<Post>,
}

impl Website {

    /// Initialize a project
    pub fn init(path: &str) -> Result<(), Box<dyn Error>> {
        if !utils::check_if_dir_empty(path)? { // Avoid overwriting existing projects/dirs
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
        let config = Config::from_file(defaults::CONFIG_FILE)?;

        Ok(Website {
            config,
            posts: utils::rlist_files(defaults::CONTENT_DIR)?
                .iter()
                //.map(|file| {println!("{}", file); return file;})
                .map(|file| Post::from_file(file).unwrap_or_default())
                .collect(),
        })
    }

}


