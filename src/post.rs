// Copyright 2024 (c) Fabian Beskow

use crate::defaults;
use crate::utils;
use crate::config::Config;

use std::fs;
use std::io::prelude::*;
use std::error::Error;
use strfmt::strfmt;
use std::collections::HashMap;

//use markdown;

/// The basic building block of a project, contains config for specific posts.
#[derive(Debug)]
pub struct Post {
    pub path: String,
    pub title: String,
    pub body: String,
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

    pub fn vec_to_html(posts: Vec<Post>, config: &Config) -> String {
        let mut post_list: String = "<ul>".to_string();
        for post in posts {
            let post_element = format!(
                "<li><a href=\"{}{}\">{}</a></li>", 
                config.base_path, 
                post.path,
                post.path,
            );
            // Remove the risk of starting a link with double slash
            let post_element = post_element.replace("//", "/"); 
            post_list.push_str(&post_element);
            //eprintln!("{}", post_element);
        }
        post_list.push_str("</ul>");

        post_list
    }

    /// Takes a config and other posts that we want indexed
    fn format_post(&self, config: &Config) -> Result<String, Box<dyn Error>> {
        let template_string = fs::read_to_string(defaults::TEMPLATE_POST_FILE)?;

        let posts = utils::rlist_files(defaults::CONTENT_DIR)?
            .iter()
            //.map(|file| {println!("{}", file); return file;})
            .map(|file| Post::from_file(file).unwrap_or_default())
            .collect::<Vec<Post>>();

        let post_list = Self::vec_to_html(posts, config);

        let mut vars = HashMap::new();

        vars.insert("title".to_string(), self.title.clone());
        vars.insert("body".to_string(), self.body.clone());
        vars.insert("blog_name".to_string(), config.blog_name.clone());
        vars.insert("posts".to_string(), post_list);

        Ok(strfmt(&template_string, &vars)?)
    }

    /// format the 
    pub fn output_to_file(&self, config: &Config, output_dir: &str) -> Result<(), Box<dyn Error>> {
        let (dir, _) = self.path.rsplit_once('/')
            .unwrap_or(("",&self.path));

        let output_dir = format!("{}/{}", output_dir, dir);

        // Create all parent dirs
        fs::create_dir_all(&output_dir)
            .or(Err(format!("Could not create dir '{}'", output_dir)))?;

        let output_filepath = format!("{}/{}", output_dir, self.path);
        eprintln!("Generating file: {}", output_filepath);

        let post_file_content = self.format_post(config)?;
        //println!("asd: {:?}", post_file_content);
        
        // Create and write file in output dir
        let mut output_file = fs::File::create(&output_filepath)?;
        output_file.write_all(&post_file_content.into_bytes())?;

        Ok(())
    }

    /// Creates a post struct from a markdown file
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut post: Post = Post::default();

        let markdown = fs::read_to_string(file_path)?;

        let Some((metadata, body)) = markdown.split_once("---") 
        else { // No metadata in markdown
            return Err(String::from("No metadata in post").into());
        };

        for line in metadata.lines() { // Might error if metadata isn't a String
            let Some((key, value)) = line.split_once(':')
            else { // no key=value pair in line
                eprintln!("Invalid metadata in line: {}", line);
                continue;
                // Err(String::from("No metadata in post").into());
            };

            match key {
                "title"       => post.title = value.into(),
                // For adding in the future:
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

        if let Some((_, path)) = file_path.split_once("content") {
            let output_filename = format!("{}.html", path
                .rsplit_once('.')
                .unwrap_or((path, "md")).0); // The filename did not include a period
            post.path = output_filename;
        } 
        post.body = markdown::to_html(body);

        Ok(post)
    }
}
