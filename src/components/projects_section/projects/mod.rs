use leptos::either::Either;
use leptos::prelude::*;
mod project_card;
use crate::components::{
    cards_list::{CardsList, CardsListMaxColCount},
    section_error::SectionError,
};
use crate::server::projects::get_projects;

use crate::icons::rust_nigeria_logo::RustNigeriaLogo;
use project_card::ProjectCard;

#[component]
pub fn Projects() -> impl IntoView {
    let projects = OnceResource::new(get_projects());

    let error = move || projects.get().and_then(|res| res.err());

    view! {
        <div class="w-full flex justify-center">
            <div class="max-w-7xl w-full relative">
                <Transition
                    fallback=move || view! {
                        <div class="h-[650px] w-full flex flex-col items-center justify-center">
                            <div class="size-20 sm:size-40 p-px flex items-center justify-center rounded-full relative overflow-hidden shadow-2xl">
                                <div
                                    style="animation-duration: 4s"
                                    class="absolute w-full top-0 left-0 h-full bg-linear-to-b from-green-300 to-70% to-grey-10 animate-spin rounded-full scale-125"
                                />
                                <div class="size-full relative bg-grey-10 rounded-full flex items-center justify-center">
                                    <RustNigeriaLogo usecase_id="events-loader" {..} class="size-16 sm:size-32 text-white animate-pulse" />
                                </div>
                            </div>
                            <p class="mt-5 title-1 animate-pulse">Loading Events...</p>
                        </div>
                     }
                >
                 <Show
                        when=move || error().is_some()
                        fallback=|| view! { }
                    >
                        {move || {
                            let err = error().unwrap();
                            view! {
                               <SectionError>
                                    {err.client_err.clone()}
                                </SectionError>
                            }
                        }}
                </Show>
                {
                    move || {
                        match projects.get() {
                            Some(Ok(data)) =>  Either::Left(
                                view! {
                                    <CardsList
                                        cards_data=move || data.clone()
                                        render_card=|project, idx| view! { <ProjectCard project=project index=idx /> }
                                        max_col_count=CardsListMaxColCount::Three
                                    />
                                }
                            ),
                            _ => Either::Right(
                                view! { <p>Failed to load</p> }
                            )
                        }.into_view()
                    }
                }
                </Transition>
            </div>
        </div>
    }
}
