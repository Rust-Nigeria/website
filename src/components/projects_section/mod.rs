mod projects;

use crate::{
    components::reveal_text_line::RevealTextLine,
    hooks::use_in_view::{use_in_view, ElementVisibilityData},
};

use leptos::html::Div;
use leptos::prelude::*;
use projects::Projects;

#[component]
pub fn ProjectsSection() -> impl IntoView {
    let section_ref = NodeRef::<Div>::new();

    let ElementVisibilityData {
        in_view: section_in_view,
    } = use_in_view(section_ref, None);

    view! {
        <div node_ref=section_ref class="text-white flex flex-col items-center bg-grey-10 py-12 px-5">
            <div class="w-full flex justify-center">
                <div class="flex flex-col w-full max-w-7xl items-start">
                    <h2 class="header-2">
                        <RevealTextLine reveal={section_in_view}>
                            Community Members Projects
                        </RevealTextLine>
                    </h2>
                    <p class="header-6 text-grey-70 text-left max-w-3xl mt-3">
                        <RevealTextLine delay=300 reveal={section_in_view}>
                           Explore Projects created by community memebers
                        </RevealTextLine>
                    </p>
                </div>
            </div>
            <Projects />
        </div>
    }
}
