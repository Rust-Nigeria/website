use leptos::prelude::*;

use crate::components::{
    community_section::CommunitySection, community_stats::CommunityStats,
    events_section::EventsSection, hero_section::HeroSection,
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="w-full">
            <HeroSection />
            <EventsSection />
            <CommunityStats />
            <CommunitySection />
        </div>
    }
}
