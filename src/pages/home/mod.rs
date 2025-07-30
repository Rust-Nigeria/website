use leptos::prelude::*;

use crate::components::{
    community_section::CommunitySection, events_section::EventsSection, hero_section::HeroSection,
    rust_features::RustFeatures,
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="w-full">
            <HeroSection />
            <EventsSection />
            <RustFeatures />
            <CommunitySection />
        </div>
    }
}
