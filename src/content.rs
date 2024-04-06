use std::collections::HashMap;

use gray_matter::{engine::YAML, Matter};
use include_dir::{include_dir, Dir};
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct PostData {
    pub meta: PostMeta,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub draft: bool,
    pub cover: String,
    pub summary: Option<String>,
}

pub fn get_all_posts() -> HashMap<String, PostData> {
    static POST_PATH: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/posts");

    POST_PATH
        .files()
        .map(|file| {
            let matter = Matter::<YAML>::new();
            let result = matter.parse(file.contents_utf8().unwrap());

            let post_meta: PostMeta = result.data.unwrap().deserialize().unwrap();

            let content = result.content;
            (
                file.path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                PostData {
                    meta: post_meta,
                    content: markdown_to_html(content),
                },
            )
        })
        .collect()
}

pub fn markdown_to_html(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    html
}

pub fn get_post(id: String) -> Option<PostData> {
    get_all_posts().get(&id).cloned()
}
