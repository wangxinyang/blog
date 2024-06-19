use leptos::*;

use crate::{
    components::{intro::Intro, post_list::PostList},
    content::get_home_page_posts,
};

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col justify-start items-center mt-24">
            <Intro/>
            <PostList posts_option=get_home_page_posts()/>
        </div>
    }
}
