// Copyright (c) Fabian Beskow 2024

use std::io::prelude::*;
use std::fs;

pub const CONTENT_DIR: &str  = "content";
pub const OUT_DIR: &str      = "out";
pub const TEMPLATE_DIR: &str = "templates";
pub const STATIC_DIR: &str   = "static";

pub const CONFIG_FILE: &str = "Config.toml";
pub const CONFIG_FILE_CONTENT: &str = r#"
# Default blog configuration file

blog_name="Example's Blog"
base_path="/blog"
domain="www.example.com"
"#;

pub const TEMPLATE_INDEX_FILE: &str = "templates/index.html";
pub const TEMPLATE_INDEX_FILE_CONTENT: &str = r#"<!DOCTYPE html>
<head>
    <title>{blog_name}</title>
</head>
<body>
    <h1>{blog_name}</h1>
    {posts}
</body>
"#;

pub const TEMPLATE_POST_FILE: &str =  "templates/post.html";
pub const TEMPLATE_POST_FILE_CONTENT: &str = r#"<!DOCTYPE html>
<html>
<head>
  <title>{title}</title>
</head>
<body>
  <h1>{title}</h1>
  {body}
 
</body>
</html>
"#;

pub fn create_default_dirs() -> std::io::Result<()> {

    fs::create_dir(OUT_DIR)?;
    fs::create_dir(CONTENT_DIR)?;
    fs::create_dir(TEMPLATE_DIR)?;
    fs::create_dir(STATIC_DIR)?;

    Ok(())
}

pub fn create_default_files() -> std::io::Result<()> {

    // None of these should error now
    { // Config file
        let mut file = fs::File::create(CONFIG_FILE)?;
        file.write_all(CONFIG_FILE_CONTENT.as_bytes())?;
    }

    { // Index template
        let mut file = fs::File::create(TEMPLATE_INDEX_FILE)?;
        file.write_all(TEMPLATE_INDEX_FILE_CONTENT.as_bytes())?;
    }

    { // Post template
        let mut file = fs::File::create(TEMPLATE_POST_FILE)?;
        file.write_all(TEMPLATE_POST_FILE_CONTENT.as_bytes())?;
    }


    Ok(())
}

