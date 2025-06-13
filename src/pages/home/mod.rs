mod community_section;

use leptos::prelude::*;

use self::community_section::CommunitySection;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="w-full">
            <section class="h-screen"></section> // This is a placeholder for the homepage hero
             <CommunitySection />
        </div>
    }
}
