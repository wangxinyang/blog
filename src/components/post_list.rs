use std::collections::HashMap;

use itertools::Itertools;
use leptos::*;

use crate::content::PostData;

#[component]
pub fn PostList(posts: HashMap<String, PostData>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-y-10 max-w-2xl mx-auto mt-40">
            {posts
                .into_iter()
                .sorted_by(|(_, prev), (_, next)| Ord::cmp(&next.meta.date, &prev.meta.date))
                .map(|(key, post)| {
                    view! { <PostItem id=key.to_string() post=post/> }
                })
                .collect_view()}

        </div>
    }
}

#[component]
fn PostItem(id: String, post: PostData) -> impl IntoView {
    view! {
        <article class="flex flex-col gap-x-4 justify-center items-start lg:flex-row">
            <div class="relative aspect-[16/9] lg:w-64 lg:shrink-0">
                <PostCover href=format!("/post/{id}") src=post.meta.cover/>
            </div>
            <div class="flex-1">
                <div class="flex flex-col justify-center items-start gap-y-2">
                    <div class="flex items-center gap-x-4">
                        <p class="text-white text-sm font-bold">
                            {chrono::DateTime::parse_from_rfc3339(&post.meta.date)
                                .unwrap()
                                .format("%e %b %Y")
                                .to_string()}
                        </p>
                        {post
                            .meta
                            .tags
                            .iter()
                            .map(|tag| view! { <TagLabel tag=tag.clone()/> })
                            .collect_view()}

                    </div>
                    <div class="">
                        <p class="text-white text-2xl font-bold">{post.meta.title}</p>
                    </div>
                    <div
                        class="prose mt-2 text-sm leading-6 text-neutral-100"
                        inner_html=post.meta.summary
                    ></div>
                </div>
            </div>
        </article>
    }
}

#[component]
fn PostCover(href: String, src: String) -> impl IntoView {
    view! {
        <a href=href>
            <img
                class="absolute h-full w-full inset-0 object-cover rounded-lg drop-shadow-sm"
                width="720"
                height="1280"
                src=src
            />
        </a>
    }
}

#[component]
fn TagLabel(tag: String) -> impl IntoView {
    view! { <p class="text-white text-sm font-bold">{tag}</p> }
}
