use leptos::prelude::*;

use super::community_section::view::CommunitySection;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="w-full">
            <section class="h-screen"></section> // This is a placeholder for the homepage hero
             <CommunitySection />
        </div>
    }
}
