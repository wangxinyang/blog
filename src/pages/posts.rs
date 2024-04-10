use leptos::*;
use leptos_meta::Title;
use leptos_router::use_params_map;

use crate::{
    components::{post_content::PostContent, post_list::PostList},
    content::{get_all_posts, get_post},
};

#[component]
pub fn Posts() -> impl IntoView {
    view! {
        <div class="text-center">
            <h1 class="mt-2 py-16 text-3xl font-bold tracking-tight text-white sm:text-4xl">
                "Posts"
            </h1>
        </div>
        <PostList posts_option=get_all_posts()/>
    }
}

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params_map();
    let id = || params.with(|params_map| params_map.get("id").cloned().unwrap_or_default());
    let post = get_post(id()).unwrap();

    view! {
        <Title text=post.meta.title.clone()/>

        <div class="mx-auto max-w-3xl px-6 lg:px-8">
            <div class="py-20 text-center">
                <p class="text-base font-semibold leading-7 text-indigo-600">
                    {chrono::DateTime::parse_from_rfc3339(&post.meta.date)
                        .unwrap()
                        .format("%e %b %Y")
                        .to_string()}
                </p>
                <h1 class="mt-2 text-3xl font-bold text-white tracking-wide sm:text-4xl">
                    {post.meta.title.clone()}
                </h1>
            </div>

            <PostContent content=post.content/>
        </div>
    }
}
