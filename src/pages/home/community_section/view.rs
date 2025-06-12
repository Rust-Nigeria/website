use crate::{
    cn,
    components::button::view::{Button, ButtonColorVariants, ButtonSizeVariants, ButtonUsecase},
    hooks::use_in_view::{use_in_view, ElementVisibilityData},
};
use leptos::html::Img;
use leptos::prelude::*;

#[component]
pub fn CommunitySection() -> impl IntoView {
    let image_el = NodeRef::<Img>::new();

    let ElementVisibilityData { in_view } = use_in_view(image_el, None);

    view! {
        <section class="pt-10 px-6 bg-background-dark py-10 w-full flex flex-col items-center">
            <div class="w-full text-grey-60">
                <p class="text-2xl lg:text-3xl">Over</p>
                <p class="text-7xl lg:text-8xl mt-3 text-grey-90 font-bold">500</p>
                <p class="mt-3 text-lg sm:text-2xl lg:text-3xl">Community Members</p>
                <p class="mt-3 text-base sm:text-xl lg:text-2xl">You are the missing piece</p>
                <Button
                    class="animate-scale-in mt-6 px-24"
                    use_as=ButtonUsecase::Link { href: String::from("https://rustnigeria.curated.co/") }
                    color=ButtonColorVariants::White
                    size=ButtonSizeVariants::Lg
                >
                    Join Us!
                </Button>
            </div>

            <div class="w-full mt-8 max-w-[858px]">
                <img node_ref={image_el} src="/assets/images/puzzle.png" class=cn!(#(
                    "aspect-[857/571] w-full opacity-0 scale-90 translate-y-20",
                    (in_view(), "opacity-100 delay-300 translate-y-0 scale-100 transition-all duration-500")
                )) />
            </div>
        </section>
    }
}
