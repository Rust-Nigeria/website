use leptos::{ev, prelude::*};

use crate::cn;
use crate::components::button::{Button, ButtonColorVariants, ButtonIconTypes, ButtonUsecase};
use crate::constants::urls;
use crate::icons::rust_nigeria_logo::RustNigeriaLogo;

#[component]
pub fn Nav() -> impl IntoView {
    let LINKS = [
        ("Blogs", urls::RUST_NIGERIA_BLOGS),
        ("Newsletter", urls::RUST_NIGERIA_NEWSLETTER),
        ("Projects", urls::RUST_NIGERIA_GITHUB),
    ];

    let (mobile_nav_open, set_mobile_nav_open) = signal(false);
    let (is_out_of_threshold, set_is_out_of_threshold) = signal(false);

    let threshold_setter = move || {
        if let Ok(scroll_y) = window().scroll_y() {
            set_is_out_of_threshold(scroll_y > 20.0);
        } else {
            set_is_out_of_threshold(false);
        }
    };

    let scroll_handle = window_event_listener(ev::scroll, move |_| threshold_setter());
    let load_handle = window_event_listener(ev::load, move |_| threshold_setter());

    on_cleanup(move || scroll_handle.remove());
    on_cleanup(move || load_handle.remove());

    view! {
        <nav class="w-full">
            <div
                class=cn!(#(
                    "w-full hidden md:flex z-40 fixed top-10 justify-center px-6 duration-300 md:bg-transparent",
                    (is_out_of_threshold(), "top-4")
                ))
            >
                <div
                    class=cn!(#(
                        "max-w-6xl flex w-full justify-between items-center duration-300 rounded-full border border-transparent",
                        (is_out_of_threshold(), "bg-background-main pl-4 pr-2 py-2 border-primary-50")
                    ))
                >
                    <a href="/" class="animate-scale-in">
                        <RustNigeriaLogo usecase_id="nav-1" {..} class="text-black size-10" />
                    </a>
                    <ul class="flex gap-x-6 items-center">
                    {
                        LINKS.into_iter().enumerate()
                            .map(|(index, (text, href))|
                                view! {
                                    <li
                                        style=format!("animation-delay: {}s", 0.3 + (index as f64 * 0.1))
                                        class="opacity-0 animate-translate-in-from-b"
                                    >
                                        <a class="hover:underline" href=href>
                                            {text}
                                        </a>
                                    </li>
                                }
                            ).collect_view()
                    }
                        <li>
                            <Button
                                class="animate-scale-in"
                                use_as=ButtonUsecase::Link { href: String::from(urls::JOIN_US) }
                                color=ButtonColorVariants::Black
                                icon=ButtonIconTypes::RightArrow
                            >
                            Join Us!
                            </Button>
                        </li>
                    </ul>
                </div>
            </div>

            <div class="md:hidden w-full relative">
                <div class="flex w-full fixed z-40 justify-between py-6 px-6 items-center bg-background-main">
                    <a href="/">
                        <RustNigeriaLogo usecase_id="nav-2" {..} class="text-black size-8" />
                    </a>

                    <button on:click=move |_| set_mobile_nav_open(true)>
                        <div class="h-6 w-6 flex flex-col justify-evenly">
                        {(0..3).into_iter().map(|_| view! {<div class="w-full h-0.5 bg-black rounded-full" />}).collect_view()}
                        </div>
                    </button>

                   <div class="flex justify-center w-full h-px absolute bottom-0 left-0 ">
                        <div class=cn!(#(
                            "w-0 h-full duration-300 origin-center bg-primary-50",
                            (is_out_of_threshold(), "w-full")
                        )) />
                   </div>
                </div>

                <div on:click=move |_| set_mobile_nav_open(false)
                    class=cn!(#(
                        "h-[100svh] w-screen z-30 fixed left-0 top-0 bg-transparent duration-500 pointer-events-none",
                        (mobile_nav_open(), "bg-grey-10/40 pointer-events-auto")
                    ))
                />
                    <div class=cn!(#("bg-background-main z-40 -translate-y-full w-full pb-8 border-b-2 border-b-primary-50 fixed left-0 top-0 duration-500", (mobile_nav_open(), "translate-y-0")))>
                        <div class="flex w-full justify-between py-6 px-6 items-center">
                            <a href="/">
                                <RustNigeriaLogo usecase_id="nav-3" {..} class="text-black size-8" />
                            </a>

                            <button class="size-6 flex justify-center items-center rounded-full border-2 border-black overflow-hidden" on:click=move |_| set_mobile_nav_open(false)>
                                <div class="size-3 relative">
                                    <div class="w-4 top-0 h-0.5 bg-black rounded-full left-0 absolute rotate-45 -translate-y-px origin-left" />
                                    <div class="w-4 top-0 h-0.5 bg-black rounded-full right-0 absolute -rotate-45 -translate-y-px origin-right" />
                                </div>
                            </button>
                        </div>

                        <ul class="flex flex-col gap-y-8 pt-4 pb-8 px-6">
                            {
                                LINKS.into_iter().enumerate()
                                    .map(|(index, (text, href))|
                                        view! {
                                            <li
                                                style=format!("transition-delay: {}s", 0.3 + (index as f64 * 0.1))
                                                class=cn!(#(
                                                    "w-full translate-y-12 opacity-0",
                                                    (mobile_nav_open(), "duration-300 translate-y-0 opacity-100")
                                                ))>
                                                <a href=href class="flex text-2xl justify-between w-full items-center">
                                                    {text}
                                                    <div class="size-3 sm:size-4 origin-center rotate-45 relative">
                                                        <div class="w-full absolute h-0.5 bg-black rounded-full top-0 right-0" />
                                                        <div class="h-full absolute w-0.5 bg-black rounded-full top-0 right-0" />
                                                        <div class="h-full w-[200%] absolute right-0 top-0">
                                                            <div class="bg-black absolute top-0 w-full -rotate-45 right-0 origin-bottom-right rounded-full h-0.5" />
                                                        </div>
                                                    </div>
                                                </a>
                                            </li>
                                        }
                                    ).collect_view()
                            }
                        </ul>
                    </div>

            </div>
        </nav>
    }
}
