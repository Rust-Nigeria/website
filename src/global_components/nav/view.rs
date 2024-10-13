use leptos::prelude::*;

use crate::global_components::button::view::{
    Button, ButtonColorVariants, ButtonIconTypes, ButtonUsecase,
};

struct NavLink<'a> {
    text: &'a str,
    href: &'a str,
}

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="max-w-6xl bg-red-300 h-14 absolute top-10 w-full mx-auto">
            <ul>
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
        </nav>
    }
}
