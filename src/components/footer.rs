use leptos::*;

use chrono::{Datelike, Utc};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full flex justify-center mt-10">
            <div class="mb-20">
                <p class="font-white text-sm text-neutral-200">
                    {format!("© {} Wang xinyang. All rights reserved.", Utc::now().year())}
                </p>
            </div>
        </footer>
    }
}
