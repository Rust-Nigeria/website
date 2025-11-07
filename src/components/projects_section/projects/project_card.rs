use crate::{cn, types::projects::CommunityProject};
use leptos::prelude::*;

#[component]
pub fn ProjectCard(
    project: CommunityProject,
    #[prop(default = "")] class: &'static str,
    #[prop(default = 0)] index: usize,
) -> impl IntoView {
    view! {
        <div class=cn!("h-full relative flex overflow-hidden flex-col w-full border border-primary-20 rounded-2xl text-left  hover:scale-95 duration-500", class)>
            <a href=project.repo_url.clone() class="relative h-[200px] min-h-[200px]">
                <img alt=project.repo_url.clone() src=project.banner class="absolute object-cover top-0 left-0 w-full h-full" />
            </a>
        </div>
    }
}
