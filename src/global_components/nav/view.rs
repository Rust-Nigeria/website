use leptos::{ev, prelude::*};

use crate::cn;
use crate::global_components::button::view::{
    Button, ButtonColorVariants, ButtonIconTypes, ButtonUsecase,
};
use crate::icons::rust_logo::RustLogo;

#[component]
pub fn Nav() -> impl IntoView {
    let LINKS = [
        ("Blogs", "/blogs"),
        ("Showcase", "/showcase"),
        ("Projects", "/projects"),
    ];

    let is_out_of_threshold = RwSignal::new(false);

    let threshold_setter = move || {
        if let Ok(scroll_y) = window().scroll_y() {
            is_out_of_threshold.set(scroll_y > 20.0);
        } else {
            is_out_of_threshold.set(false);
        }
    };

    let scroll_handle = window_event_listener(ev::scroll, move |_| threshold_setter());
    let load_handle = window_event_listener(ev::load, move |_| threshold_setter());

    on_cleanup(move || scroll_handle.remove());
    on_cleanup(move || load_handle.remove());

    view! {
        <nav
            class=cn!(
                #(is_out_of_threshold, "md:top-4"),
                "w-full flex fixed md:top-10 justify-center md:px-6 duration-300 bg-background-main md:bg-transparent",
            )
        >
            <div
                class=cn!(
                    #(is_out_of_threshold, "bg-background-main pl-4 pr-2 py-2 border-primary-50"),
                    "max-w-6xl w-full justify-between items-center duration-300 rounded-full border border-transparent hidden md:flex"
                )
            >
                <a href="/">
                    <RustLogo {..} class="text-black size-8" />
                </a>
                <ul class="flex gap-x-6 items-center">
                {
                    LINKS.into_iter()
                        .map(|(text, href)|
                            view! {
                                <li><a class="hover:underline" href=href>{text}</a></li>
                            }
                        ).collect_view()
                }
                    <li>
                        <Button
                            use_as=ButtonUsecase::Link { href: String::from("youtube.com") }
                            color=ButtonColorVariants::Black
                            icon=ButtonIconTypes::RightArrow
                        >
                        Join Us
                        </Button>
                    </li>
                </ul>
            </div>
            <div class="md:hidden w-full relative px-6">
                <div class="flex w-full justify-between py-4 items-center">
                    <a href="/">
                        <RustLogo {..} class="text-black size-8" />
                    </a>

                    <button>
                      <div class="h-6 w-6 flex flex-col justify-evenly">
                      {(0..3).into_iter().map(|_| view! {<div class="w-full h-0.5 bg-black rounded-full" />}).collect_view()}
                      </div>
                    </button>
                </div>
                <div class=cn!(
                    #(is_out_of_threshold, "w-full"),
                    "absolute bottom-0 w-0 h-px duration-300 origin-center left-0 bg-primary-50"
                ) />
            </div>
        </nav>
    }
}
