use leptos::prelude::*;
mod event_card;
use crate::components::events_section::constants::EVENTS;
use crate::hooks::use_in_view::ElementVisibilityData;

use crate::{cn, hooks::use_in_view::use_in_view};

use event_card::EventCard;
use leptos::html::Div;

#[component]
pub fn EventsList() -> impl IntoView {
    let section_ref = NodeRef::<Div>::new();

    let ElementVisibilityData {
        in_view: section_in_view,
    } = use_in_view(section_ref, None);

    view! {
        <div node_ref=section_ref class="mt-8 px-6 w-full flex justify-center no-scrollbar overflow-x-auto">
            <div class="max-w-7xl w-full relative">
                <div class="flex h-full gap-x-4 pr-6">
                    {EVENTS.into_iter().enumerate().map(|(index, event)| view! {
                            <div style=format!("transition-delay: {}s", (index as f32) * 0.1) class=cn!(#("opacity-0 duration-500 translate-x-10", (section_in_view(), "opacity-100 translate-x-0")))>
                                <EventCard event=event />
                            </div>
                        }).collect_view()
                    }
                </div>
            </div>
        </div>
    }
}
