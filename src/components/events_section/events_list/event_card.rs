use crate::{
    cn,
    components::{
        button::{Button, ButtonColorVariants, ButtonIconTypes, ButtonSizeVariants, ButtonUsecase},
        events_section::types::RustEvent,
    },
};
use leptos::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Date;

fn parse_iso_js(iso_str: &str) -> String {
    let date = Date::new(&JsValue::from_str(iso_str));
    date.to_locale_string("en-US", &JsValue::undefined()).into()
}

#[component]
pub fn EventCard(event: RustEvent, #[prop(default = "")] class: &'static str) -> impl IntoView {
    // let date_string = signal(parse_iso_js("2025-07-11T15:00:00Z"));
    view! {
        <div class=cn!("h-full flex flex-col w-[375px] border border-grey-20 bg-background-dark rounded-2xl text-left p-4", class)>
            <img src=event.banner alt=event.name class="h-[213px] object-cover rounded-lg overflow-hidden" />

            <h3 class="mt-6 title-1">{event.name}</h3>

            <p class="text-grey-60 body-2 mt-2 line-clamp-3">{event.description}</p>

            <div class="flex items-center gap-x-2 mt-2">
                <span class="font-semibold">SPEAKERS:</span>
                <div class="flex">
                    {event.speakers.into_iter().enumerate().map(|(index, speaker)| view! {
                        <img style=format!("transform: translateX(-{}%)", index * 30) class="size-8 border-2 border-white rounded-full" src=speaker.image />
                    }).collect_view()}
                </div>
            </div>

            <Button
                class="mt-6 w-fit"
                use_as=ButtonUsecase::Link { href: String::from(event.event_link) }
                color=ButtonColorVariants::Grey
                size=ButtonSizeVariants::Md
                icon={ButtonIconTypes::RightArrow}
            >
                Learn More
            </Button>
        </div>
    }
}
