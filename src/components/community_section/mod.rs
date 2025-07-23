mod members_jigsaw;

use crate::constants::urls;
use crate::{
    cn,
    components::{
        button::{Button, ButtonColorVariants, ButtonSizeVariants, ButtonUsecase},
        reveal_text_line::RevealTextLine,
    },
    hooks::use_in_view::{use_in_view, ElementVisibilityData},
};
use leptos::html::{Div, Img};
use leptos::prelude::*;

#[component]
pub fn CommunitySection() -> impl IntoView {
    let section_ref = NodeRef::<Div>::new();
    let image_ref = NodeRef::<Img>::new();

    let ElementVisibilityData {
        in_view: section_in_view,
    } = use_in_view(section_ref, None);
    let ElementVisibilityData {
        in_view: image_in_view,
    } = use_in_view(image_ref, None);

    view! {
        <section class="pt-10 px-6 bg-background-dark py-10 w-full flex flex-col items-center">
            <div node_ref=section_ref class="w-full text-grey-60">
                <p class="text-2xl lg:text-3xl">
                    <RevealTextLine reveal={section_in_view}>
                        Over
                    </RevealTextLine>
                </p>
                <p class="text-7xl lg:text-8xl mt-3 text-grey-90 font-bold">
                    <RevealTextLine delay={100} reveal={section_in_view}>
                        500
                    </RevealTextLine>
                </p>
                <p class="mt-3 text-lg sm:text-2xl lg:text-3xl">
                    <RevealTextLine delay={200} reveal={section_in_view}>
                        Community Members
                    </RevealTextLine>
                </p>
                <p class="mt-3 text-base sm:text-xl lg:text-2xl">
                    <RevealTextLine delay={300} reveal={section_in_view}>
                       You are the missing piece
                    </RevealTextLine>
                </p>
                <RevealTextLine delay={400} reveal={section_in_view}>
                    <Button
                        class="animate-scale-in mt-6 px-20 2xl:px-24"
                        use_as=ButtonUsecase::Link { href: String::from(urls::JOIN_US) }
                        color=ButtonColorVariants::White
                        size=ButtonSizeVariants::Lg
                    >
                        Join Us!
                    </Button>
                </RevealTextLine>
            </div>

            <div class="w-full mt-8 max-w-[858px]">
                <img node_ref=image_ref src="/assets/images/puzzle.png" class=cn!(#(
                    "aspect-[857/571] w-full opacity-0 scale-90 translate-y-20",
                    (image_in_view(), "opacity-100 translate-y-0 scale-100 transition-all duration-1000")
                )) />
            </div>
        </section>
    }
}
