use crate::pages::home::community_section::view::CommunitySection;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="w-full">
            <section class="h-screen"></section> // This is a placeholder for the homepage her
             <CommunitySection />
        </div>
    }
}
