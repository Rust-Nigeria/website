mod events;

use crate::{
    components::reveal_text_line::RevealTextLine,
    hooks::use_in_view::{use_in_view, ElementVisibilityData},
};
use events::Events;
use leptos::html::Div;
use leptos::prelude::*;

#[component]
pub fn EventsSection() -> impl IntoView {
    let section_ref = NodeRef::<Div>::new();

    let ElementVisibilityData {
        in_view: section_in_view,
    } = use_in_view(section_ref, None);

    view! {
        <div node_ref=section_ref class="text-white flex flex-col items-center bg-grey-10 py-12 px-5 gap-y-10">
            <div class="w-full flex justify-center">
                <div class="flex flex-col w-full max-w-7xl items-start">
                    <h2 class="header-2">
                        <RevealTextLine reveal={section_in_view}>
                            Rust Events
                        </RevealTextLine>
                    </h2>
                    <p class="header-6 text-grey-70 text-left max-w-3xl mt-3">
                        <RevealTextLine delay=300 reveal={section_in_view}>
                            Mark your calendars for our upcoming seminars, meetups and hangouts!
                        </RevealTextLine>
                    </p>
                </div>
            </div>
            <Events />
        </div>
    }
}
