use leptos::prelude::*;

use crate::global_components::button::view::{
    Button, ButtonColorVariants, ButtonIconTypes, ButtonUsecase,
};

#[component]
pub fn Nav() -> impl IntoView {
    let LINKS = vec![
        (String::from("Blogs"), String::from("/blogs")),
        (String::from("Showcase"), String::from("/showcase")),
        (String::from("Projects"), String::from("/projects")),
    ];

    view! {
        <nav class="max-w-6xl h-14 absolute top-10 w-full mx-auto">
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
        </nav>
    }
}
