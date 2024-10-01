// Copyright (c) Fabian Beskow 2024

use crate::Post;
use crate::utils;

use std::error::Error;

#[derive(Debug)]
pub struct Index {
    directory: String, // Index of ...
    //name: String,
    posts: Vec<Post>,
}

impl Index {
    pub fn from_content_dir(directory: &str) -> Result<Index, Box<dyn Error>> {
        let post_files = utils::rlist_files(directory)?;

        let posts: Vec<Post> = post_files
            .into_iter()
            .map(|file| Post::from_file(&file)) // This creates a vec of errors
            .collect::<Result<Vec<_>, _>>()?; // Return the potential error
                                              //
        Ok(Index {
            directory: directory.into(),
            posts
        })
    }
}

