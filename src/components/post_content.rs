use leptos::*;

#[component]
pub fn PostContent(content: String) -> impl IntoView {
    view! { <div class="prose prose-slate max-w-none" inner_html=content></div> }
}
