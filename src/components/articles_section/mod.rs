mod articles;

use crate::{
    components::reveal_text_line::RevealTextLine,
    hooks::use_in_view::{use_in_view, ElementVisibilityData},
};

use articles::Articles;
use leptos::html::Div;
use leptos::prelude::*;

#[component]
pub fn ArticlesSection() -> impl IntoView {
    let section_ref = NodeRef::<Div>::new();

    let ElementVisibilityData {
        in_view: section_in_view,
    } = use_in_view(section_ref, None);

    view! {
        <div node_ref=section_ref class="text-grey-10 flex flex-col items-center bg-grey-100 py-12 px-5 gap-y-2">
            <div class="w-full flex justify-center">
                <div class="flex flex-col w-full max-w-7xl items-start">
                    <h2 class="header-2">
                        <RevealTextLine reveal={section_in_view}>
                            Community Articles
                        </RevealTextLine>
                    </h2>
                    <p class="header-6 text-grey-40 text-left max-w-3xl mt-3">
                        <RevealTextLine delay=300 reveal={section_in_view}>
                           Read articles written by members of our community!
                        </RevealTextLine>
                    </p>
                </div>
            </div>
            <Articles />
        </div>
    }
}
