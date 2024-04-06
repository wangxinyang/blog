use leptos::*;

use crate::content::markdown_to_html;

#[component]
pub fn PostContent(content: String) -> impl IntoView {
    view! {
        <article
            class="prose prose-zinc max-w-none text-neutral-100"
            inner_html=markdown_to_html(content)
        ></article>
    }
}
