// Copyright (c) Fabian Beskow 2024

use std::fs;
use std::error::Error;

/// Recursivly list files in directory this might break if recursivly symlinked
pub fn rlist_files(dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
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
