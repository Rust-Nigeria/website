mod stats;

use leptos::prelude::*;

use leptos::html::Div;
use stats::Stats;

use crate::constants::urls;
use crate::{
    cn,
    components::button::{Button, ButtonColorVariants, ButtonSizeVariants, ButtonUsecase},
    hooks::use_in_view::{use_in_view, ElementVisibilityData},
};

#[component]
pub fn CommunityStats() -> impl IntoView {
    let section_ref = NodeRef::<Div>::new();
    let ElementVisibilityData {
        in_view: section_in_view,
    } = use_in_view(section_ref, None);
    view! {
        <div class="bg-background-dark text-grey-100 px-6 py-12 sm:py-14 gap-y-8 flex flex-col w-full items-center">
           <div node_ref=section_ref class="w-full max-w-7xl flex flex-col items-center">
                <h2 class=cn!(#(
                    "header-2 text-center max-w-4xl opacity-0 translate-y-8 duration-500",
                     (section_in_view(), "opacity-100 translate-y-0")
                ))>
                    We have fallen in love with Rust for many reasons
                </h2>
                <p class=cn!(#(
                    "mt-3 text-center header-6 text-grey-70 max-w-3xl opacity-0 translate-y-8 duration-500",
                     (section_in_view(), "opacity-100 translate-y-0 delay-200")
                ))>
                    Numerous surveys, awards won, the resilient community backing the language are among the reasons we have fallen in love with Rust
                </p>
                <div class=cn!(#(
                    "opacity-0 translate-y-8 duration-500 mt-6",
                     (section_in_view(), "opacity-100 translate-y-0 delay-300")
                ))>
                    <Button
                        use_as=ButtonUsecase::Link { href: String::from(urls::JOIN_US) }
                        color=ButtonColorVariants::White
                        size=ButtonSizeVariants::Lg
                    >
                        Join the Community
                    </Button>
                </div>
           </div>
           <Stats />
        </div>
    }
}
