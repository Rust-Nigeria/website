mod cards;

use leptos::prelude::*;

use crate::components::button::{Button, ButtonColorVariants, ButtonSizeVariants, ButtonUsecase};
use crate::constants::urls;
use cards::Cards;

#[component]
pub fn RustFeatures() -> impl IntoView {
    view! {
        <div class="bg-background-dark text-grey-100 px-6 py-10 flex flex-col w-full items-center">
           <div class="w-full max-w-7xl flex flex-col items-start">
                <h2 class="header-2 text-left max-w-4xl">
                    We have fallen in love with Rust for many reasons
                </h2>
                <p class="mt-3 text-left header-6 text-grey-70 max-w-3xl">
                    Numerous surveys, awards won, the resilient community backing the language are among the reasons we have fallen in love with Rust
                </p>
                 <Button
                    class="animate-scale-in mt-6"
                    use_as=ButtonUsecase::Link { href: String::from(urls::JOIN_US) }
                    color=ButtonColorVariants::White
                    size=ButtonSizeVariants::Lg
                >
                    Join the Community
                </Button>
           </div>
           <Cards />
        </div>
    }
}
