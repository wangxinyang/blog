use icondata as i;
use icondata_core::Icon;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center items-center mb-4">
            <h1 class="text-white lg:text-7xl sm:text-4xl font-bold">
                "Hey! there. I'm Wang xinyang👋"
            </h1>
            <p class="mt-2 sm:text-lg lg:text-xl leading-8 text-white p-8">
                "Welcome to my website, where I write about Java, Golang, Rust, React, Nodejs, and more!"
            </p>
            <SocialIcons/>
        </div>
    }
}

struct Social<'a> {
    name: &'a str,
    url: &'a str,
    icon: Icon,
}

#[component]
fn SocialIcons() -> impl IntoView {
    let socials = [
        Social {
            name: "Mail",
            url: "mailto:mail@wangxinyang1983@gmail.com",
            icon: i::AiMailFilled,
        },
        Social {
            name: "GitHub",
            url: "https://github.com/wangxinyang",
            icon: i::AiGithubFilled,
        },
        Social {
            name: "Twitter",
            url: "https://twitter.com/toseiw",
            icon: i::AiTwitterOutlined,
        },
    ];

    view! {
        <div class="mt-4 flex justify-center space-x-5">
            {socials
                .iter()
                .map(|social| {
                    view! {
                        <a
                            key=social.name
                            href=social.url
                            target="_blank"
                            class="text-gray-400 hover:text-gray-500"
                        >
                            <span class="sr-only">{social.name}</span>
                            <Icon icon=social.icon class="h-6 w-6 text-white"/>
                        </a>
                    }
                })
                .collect_view()}
        </div>
    }
}
