use leptos::prelude::*;

use crate::global_components::button::view::{
    Button, ButtonColorVariants, ButtonIconTypes, ButtonUsecase,
};
use crate::icons::rust_logo::RustLogo;

#[component]
pub fn Nav() -> impl IntoView {
    let LINKS = [
        ("Blogs", "/blogs"),
        ("Showcase", "/showcase"),
        ("Projects", "/projects"),
    ];

    view! {
        <nav class="w-full flex absolute top-10 justify-center">
          <div class="max-w-6xl w-full flex justify-between items-center">
            <a href="/">
                <RustLogo {..} class="text-black size-9" />
            </a>
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
            </div>
        </nav>
    }
}
