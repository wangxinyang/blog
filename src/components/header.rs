use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let nav_items = [("About", "/about"), ("Posts", "/posts"), ("Tags", "/tags")];

    view! {
        <header class="w-full m-4">
            <div class="flex justify-evenly items-center">
                <a href="/">
                    <span class="sr-only">"Wang xinyang"</span>
                    <img
                        class="h-10 w-auto rounded-full"
                        src="/images/avatar.png"
                        alt="avatar"
                        href="/"
                    />
                </a>
                <div class="flex gap-x-6 lg:gap-x-12">
                    {nav_items
                        .iter()
                        .map(|(name, href)| {
                            view! {
                                <a
                                    class="text-sm font-semibold leading-6 text-gray-900"
                                    href=href.to_string()
                                >
                                    {name.to_string()}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </header>
    }
}
