use crate::{
    cn,
    components::button::view::{Button, ButtonColorVariants, ButtonSizeVariants, ButtonUsecase},
};
use leptos::prelude::*;

use leptos::html::Img;
use leptos_use::use_intersection_observer;

#[component]
pub fn CommunitySection() -> impl IntoView {
    let image_el = NodeRef::<Img>::new();
    let (is_image_visible, set_is_image_visible) = signal(false);

    use_intersection_observer(image_el, move |entries, _| {
        set_is_image_visible(entries[0].is_intersecting());
    });

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
                    "aspect-[857/571] w-full opacity-0 translate-y-20",
                    (is_image_visible(), "opacity-100 translate-y-0 transition-all duration-500")
                )) />
            </div>
        </section>
    }
}
