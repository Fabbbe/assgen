// Copyright (c) Fabian Beskow 2024

use std::fs;
use std::error::Error;
use std::path::Path;
//use markdown;

/// The basic building block of a project, contains config for specific posts.
pub struct Post {
    path: String,
    title: String,
    body: String,
}

impl Post {
    /// Creates a post struct from a markdown file
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {

        let mut title: String = "untitled".into();

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
                "title"       => title = value.into(),
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

        let path: String = file_path.into();
        let body: String = markdown::to_html(body);

        Ok(Post {
            path,
            title,
            body,
        })
    }
}
