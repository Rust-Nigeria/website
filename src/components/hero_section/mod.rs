mod member_pointers;

use leptos::html::Div;
use leptos::prelude::*;

use crate::cn;
use crate::components::button::{
    Button, ButtonColorVariants, ButtonIconTypes, ButtonSizeVariants, ButtonUsecase,
};
use crate::components::reveal_text_line::RevealTextLine;
use crate::hooks::use_in_view::{use_in_view, ElementVisibilityData};
use crate::icons::nigeria_flag::NigeriaFlag;
use member_pointers::MemberPointers;

#[component]
pub fn HeroSection() -> impl IntoView {
    let wrapper_el = NodeRef::<Div>::new();

    let ElementVisibilityData { in_view } = use_in_view(wrapper_el, None);

    view! {
        <div node_ref=wrapper_el class="pt-40 px-6 pb-56 relative flex justify-center bg-background-main">
            <MemberPointers />
            <div class="w-full max-w-7xl relative flex items-center flex-col">
                <div class=cn!(#(
                    "px-6 py-2 duration-300 bg-success-90 rounded-full w-fit flex items-center gap-2 title-1 scale-x-0",
                    (in_view(), "scale-x-100")
                ))>
                    <NigeriaFlag {..} class=cn!(#(
                        "w-8 h-6 opacity-0 duration-300 scale-0 delay-200",
                        (in_view(), "opacity-100 scale-100")
                    )) />
                    <RevealTextLine reveal=in_view delay=200>Proudly Nigerian</RevealTextLine>
                </div>
                <h1 class="header-1 flex flex-col items-center text-center max-w-4xl mt-4">
                    <RevealTextLine reveal=in_view>Your #1 Community for</RevealTextLine>
                    <RevealTextLine reveal=in_view delay=200> Rust Enthusiasts</RevealTextLine>
                </h1>
                <p class="header-6 text-grey-50 text-center max-w-4xl mt-4">
                    <RevealTextLine reveal=in_view delay=200>Welcome to the community for the best language on the planet,</RevealTextLine>
                    <RevealTextLine reveal=in_view delay=400> insights, top quality memes and more run aplenty here</RevealTextLine>
                </p>
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
