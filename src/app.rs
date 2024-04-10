use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::pages::about::About;
use crate::pages::home::Home;
use crate::pages::posts::{Post, Posts};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" class="bg-red-400"/>
        <Stylesheet id="leptos" href="/pkg/my-blog.css"/>
        <Link rel="shortcut icon" type_="image/png" href="/favicon.ico"/>

        // sets the document title
        <Title text="Wangxy's Blog"/>

        <Header/>
        <div class="flex-1">
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }.into_view()
            }>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/about" view=About/>
                    <Route path="/posts" view=Posts/>
                    <Route path="/post/:id" view=Post/>
                </Routes>
            </Router>
        </div>
        <Footer/>
    }
}
