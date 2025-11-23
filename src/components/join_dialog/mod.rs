use leptos::prelude::*;

use crate::{
    cn,
    components::button::{
        Button, ButtonColorVariants, ButtonIconTypes, ButtonSizeVariants, ButtonUsecase,
    },
    constants::urls,
    hooks::use_join_community_dialog::use_join_community_dialog,
    types::dialog::DialogState,
};

#[component]
pub fn JoinDialog() -> impl IntoView {
    let (_, close_fn, state) = use_join_community_dialog();

    let on_close = move |_| close_fn();

    let should_show = move || !(matches!(state(), DialogState::Closed));

    view! {
        <div class=cn!(#(
            "h-svh w-screen hidden fixed z-40 items-center justify-center p-5",
            (should_show(), "flex")
        ))>
            <button
                on:click=on_close
                class=cn!(#(
                    "absolute bg-grey-20/80 top-0 left-0 duration size-full",
                        (matches!(state(), DialogState::Opening), "animate-fade-in"),
                        (matches!(state(), DialogState::Closing), "animate-fade-out")
                ))
            />
            <div class=cn!(#(
                "relative border-primary-70 border bg-white rounded-2xl p-8 w-full max-w-[400px]",
                (matches!(state(), DialogState::Opening), "animate-scale-in-95"),
                (matches!(state(), DialogState::Closing), "animate-fade-out")
            ))>
                <h3 class="header-6">Join our Community</h3>
                <p class="mt-4">
                    "Our community is most active on WhatsApp.
        To protect members’ privacy, we add new members" <b> manually. </b>
                </p>
                <p class="mt-2">"Please send us a message on" <a href=urls::RUST_NIGERIA_X class="link"> X (Twitter) </a> "and we’ll get you added!"</p>
                <Button
                    class="mt-6"
                    use_as=ButtonUsecase::Link { href: String::from(urls::RUST_NIGERIA_X) }
                    color=ButtonColorVariants::Grey
                    size=ButtonSizeVariants::Md
                    icon={ButtonIconTypes::RightArrow}
                >
                Message us on X
                </Button>
            </div>
        </div>
    }
}
