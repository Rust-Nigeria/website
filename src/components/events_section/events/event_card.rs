use crate::{
    cn,
    components::button::{
        Button, ButtonColorVariants, ButtonIconTypes, ButtonSizeVariants, ButtonUsecase,
    },
    types::events::CommunityEvent,
    utils::parse_iso_js::parse_iso_js,
};
use leptos::prelude::*;

fn get_color_pair(index: usize) -> (String, String) {
    let hue = (index as f32 * 47.0) % 360.0; // deterministic hue spread
    let bg = format!("hsl({hue:.0}, 70%, 85%)"); // light background
    let text = format!("hsl({hue:.0}, 70%, 25%)"); // darker text
    (bg, text)
}

#[component]
pub fn EventCard(
    event: CommunityEvent,
    #[prop(default = "")] class: &'static str,
    #[prop(default = 0)] index: usize,
) -> impl IntoView {
    let date = RwSignal::new(None);

    Effect::new(move || {
        date.set(Some(parse_iso_js(&event.date.to_rfc3339())));
    });

    let (bg, text) = get_color_pair(index);

    view! {
        <div class=cn!("h-full relative flex flex-col w-full border border-grey-20 bg-background-dark rounded-2xl text-left p-4", class)>
            <div style=format!("background-color: {}; color: {};", bg, text) class="absolute left-0 text-sm px-2 py-1 font-medium rounded-lg top-0 -translate-y-2/5">{date}</div>
            <img src=event.banner alt=event.name.clone() class="h-[213px] object-cover rounded-lg overflow-hidden" />

            <h3 class="mt-4 title-1 line-clamp-2 h-14">{event.name.clone()}</h3>

            <p class="text-grey-60 body-2 mt-1 line-clamp-3">{event.description}</p>

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
                use_as=ButtonUsecase::Link { href: event.event_link }
                color=ButtonColorVariants::Grey
                size=ButtonSizeVariants::Md
                icon={ButtonIconTypes::RightArrow}
            >
                Learn More
            </Button>
        </div>
    }
}
