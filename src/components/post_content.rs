use leptos::*;

use pulldown_cmark::{html, Options, Parser};

#[component]
pub fn PostContent(content: String) -> impl IntoView {
    view! {
        <article class="prose prose-zinc max-w-none" inner_html=markdown_to_html(content)></article>
    }
}

fn markdown_to_html(content: String) -> String {
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
