use leptos::prelude::*;

use crate::components::{
    articles_section::ArticlesSection, community_stats::CommunityStats,
    events_section::EventsSection, hero_section::HeroSection,
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="w-full">
            <HeroSection />
            <EventsSection />
            <CommunityStats />
            <ArticlesSection />
        </div>
    }
}
