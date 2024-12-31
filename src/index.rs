// Copyright 2024 (c) Fabian Beskow

use crate::post::Post;
use crate::config::Config;
use crate::utils;
use crate::defaults;
use crate::templates;

use std::fs;
use std::error::Error;

pub struct Index {
    //path: String,
    title: String,
    posts: Vec<Post>,
}

impl Default for Index {
    fn default() -> Self {
        Index {
            title: "Utitled index".into(),
            posts: vec![],
        }
    }
}

impl Index {
    pub fn from_posts(title: &str, posts: Vec<Post>) -> Self {
        Index {
            title: title.into(),
            posts,
        }
    }

    pub fn from_path(title: &str, path: &str) -> Result<Index, Box<dyn Error>> {

        let path = format!("{}/{}", defaults::CONTENT_DIR, path);

        eprintln!("Generating index {}", path);

        let posts: Vec<Post> = utils::rlist_files(&path)?
            .into_iter()
            .map(|file| Post::from_file(&file).unwrap_or_default())
            .collect();

        Ok(Self::from_posts(title, posts))
    }

    pub fn to_html(&self, config: &Config) -> String {

        templates::gen_post(&self.title, &Post::vec_to_html(self.posts.clone(), config), "AAAAAAAAA")
    }
}

