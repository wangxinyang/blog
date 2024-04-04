use leptos::*;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center items-center mb-4">
            <h1 class="text-white lg:text-7xl sm:text-4xl font-bold">
                "Hey! there. I'm Wangxy👋"
            </h1>
            <p class="hidden mt-2 text-lg leading-8 text-white lg:block p-8">
                "Welcome to my website, where I write about Java, Golang, Rust, React, Nodejs, and more!"
            </p>
        </div>
    }
}
