use leptos::prelude::*;
use web_sys::js_sys::Date;
mod event_card;
mod events_list;
use crate::server::events::get_events;

use crate::icons::rust_nigeria_logo::RustNigeriaLogo;
use chrono::Utc;
use events_list::EventsList;

#[component]
pub fn Events() -> impl IntoView {
    let events = LocalResource::new(move || {
        let today_str = if cfg!(target_arch = "wasm32") {
            let date: String = Date::new_0().to_iso_string().into();
            date
        } else {
            Utc::now().to_rfc3339()
        };

        get_events(today_str)
    });

    let past_events = move || events.get().and_then(Result::ok).map_or(vec![], |v| v.past);
    let upcoming_events = move || {
        events
            .get()
            .and_then(Result::ok)
            .map_or(vec![], |v| v.upcoming)
    };

    view! {
        <div class="w-full flex justify-center">
            <div class="max-w-7xl w-full relative">
                <Transition
                    fallback=move || view! {
                        <div class="h-[650px] w-full flex flex-col items-center justify-center">
                            <div class="size-20 sm:size-40 p-px flex items-center justify-center rounded-full relative overflow-hidden shadow-2xl">
                                <div
                                    style="animation-duration: 4s"
                                    class="absolute w-full top-0 left-0 h-full bg-linear-to-b from-green-300 to-70% to-grey-10 animate-spin rounded-full scale-125"
                                />
                                <div class="size-full relative bg-grey-10 rounded-full flex items-center justify-center">
                                    <RustNigeriaLogo usecase_id="events-loader" {..} class="size-16 sm:size-32 text-white animate-pulse" />
                                </div>
                            </div>
                            <p class="mt-5 title-1 animate-pulse">Loading Events...</p>
                        </div>
                     }
                >

                    <div class="flex flex-col gap-y-8">
                        <Show
                            when=move || !upcoming_events().is_empty()
                        >
                            <EventsList title="Upcoming Events" events=upcoming_events />
                        </Show>
                        <EventsList title="Past Events" events=past_events />
                    </div>
                </Transition>
            </div>
        </div>
    }
}
