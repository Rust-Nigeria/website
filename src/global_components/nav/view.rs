use leptos::*;

use crate::global_components::button::view::{
    Button, ButtonColorVariants, ButtonSizeVariants, ButtonUsecase,
};

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="max-w-6xl absolute top-10 w-full mx-auto">
            <ul>
                <li>
                    <Button
                        usecase=ButtonUsecase::Link { href: String::from("youtube.com") }
                        color=ButtonColorVariants::Black
                    >
                    Hello
                    </Button>
                </li>
            </ul>
        </nav>
    }
}
