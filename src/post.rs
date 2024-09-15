// Copyright 2024 (c) Fabian Beskow

use std::fs;
use std::error::Error;
use std::path::Path;
//use markdown;

/// The basic building block of a project, contains config for specific posts.
#[derive(Debug)]
pub struct Post {
    path: String,
    title: String,
    body: String,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            title: "untitled".into(),
            path: "undefined".into(),
            body: "undefined".into(),
        }
    }
}

impl Post {
    /// Creates a post struct from a markdown file
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut post: Post = Post::default();

        let markdown = fs::read_to_string(file_path)?;

        let Some((metadata, body)) = markdown.split_once("---") 
        else { // No metadata in markdown
            return Err(String::from("No metadata in post").into());
        };

        for line in metadata.lines() { // Might error since metadata isn't a String
            let Some((key, value)) = line.split_once('=')
            else { // no key=value pair in line
                eprintln!("Invalid metadata in line: {}", line);
                continue;
                // Err(String::from("No metadata in post").into());
            };

            match key {
                "title"       => post.title = value.into(),
                /*
                "cover_image" => out_post.cover_image = value.into(),
                "keywords"    => {
                    let keywords: Vec<String> = value.split(",")
                        .map(|v| 
                            String::from(v)
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .join("")
                        )
                        .collect();
                    out_post.keywords = keywords;
                },
                "date_posted" => out_post.date_posted = DateTime::parse_from_rfc3339(value)?
                                    .to_utc(),
                */
                _             => eprintln!("{} is not a valid key", key),
            } 
        }

        post.path = file_path.into();
        post.body = markdown::to_html(body);

        Ok(post)
    }
}
