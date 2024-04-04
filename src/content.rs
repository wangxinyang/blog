use std::{collections::HashMap, env};

use gray_matter::{engine::YAML, Matter};
use include_dir::{include_dir, Dir};

pub struct PostData {
    meta: PostMeta,
    content: String,
}

pub struct PostMeta {
    title: String,
    date: String,
    tags: Vec<String>,
    draft: bool,
    cover: String,
    summary: String,
}

pub fn get_all_posts() -> HashMap<String, PostData> {
    static POST_PATH: Dir = include_dir!("./content/posts");
    POST_PATH.files().map(|file| -> () {
        let matter = Matter::<YAML>::new();
        let result = matter.parse(file.contents_utf8().unwrap());

        ()
    });
    HashMap::new()
}
