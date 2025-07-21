use leptos::prelude::*;

use crate::components::{
    community_section::CommunitySection, events_section::EventsSection, hero_section::HeroSection,
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="w-full">
            <HeroSection />
            <EventsSection />
            <CommunitySection />
        </div>
    }
}
