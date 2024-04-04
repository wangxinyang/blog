use leptos::*;

use crate::components::intro::Intro;

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full h-full flex flex-col justify-start items-center mt-24">
            <Intro/>
            <p>Home</p>
        </div>
    }
}
