
use crate::defaults;

use strfmt::strfmt;

use std::fs;
use std::collections::HashMap;

pub fn gen_post(title: &str, body: &str, blog_name: &str) -> String {
    
    let template_string = fs::read_to_string(defaults::TEMPLATE_POST_FILE)
        .unwrap_or(defaults::TEMPLATE_POST_FILE_CONTENT.to_string());

    let mut vars = HashMap::new();

    vars.insert("title".to_string(), title);
    vars.insert("body".to_string(), body);
    vars.insert("blog_name".to_string(), blog_name);

    strfmt(&template_string, &vars).unwrap()
}
