use leptos::prelude::*;

#[component]
pub fn CommunitySection() -> impl IntoView {
    view! {
        <section class="pt-10 bg-background-dark mt-10 w-full flex flex-col items-center">
            <div class="max-w-60 w-full text-grey-60">
                <p class="text-2xl">Over</p>
                <p class="text-8xl text-grey-90 font-bold">400</p>
            </div>
        </section>
    }
}
