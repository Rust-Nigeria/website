use leptos::prelude::*;

use crate::components::button::{
    Button, ButtonColorVariants, ButtonIconTypes, ButtonSizeVariants, ButtonUsecase,
};
use crate::icons::nigeria_flag::NigeriaFlag;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <div class="pt-40 px-6 pb-56 flex justify-center bg-background-main">
            <div class="w-full max-w-7xl flex items-center flex-col">
                <div class="px-6 py-2 bg-success-90 rounded-full w-fit flex items-center gap-2 title-1">
                    <NigeriaFlag {..} class="w-8 h-6" />Proudly Nigerian
                </div>
                <h1 class="header-1 text-center max-w-4xl mt-4">Your #1 Community for Rust Enthusiasts</h1>
                <p class="header-6 text-grey-50 text-center max-w-4xl mt-4">Welcome to the community for the best language on the planet, insights, top quality memes and more run aplenty here</p>
                <Button
                    class="animate-scale-in mt-6"
                    use_as=ButtonUsecase::Link { href: String::from("/") }
                    color=ButtonColorVariants::Black
                    icon=ButtonIconTypes::RightArrow
                    size={ButtonSizeVariants::Lg}
                >
                   Become One of Us
                </Button>
            </div>
        </div>
    }
}
