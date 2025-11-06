use leptos::either::Either;
use leptos::prelude::*;
mod article_card;
use crate::components::cards_list::CardsList;
use crate::server::articles::get_articles;

use crate::icons::rust_nigeria_logo::RustNigeriaLogo;
use article_card::ArticleCard;

#[component]
pub fn Articles() -> impl IntoView {
    let articles = OnceResource::new(get_articles());

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


                    {
                        move || {
                            match articles.get() {
                                Some(Ok(data)) =>  Either::Left(
                                    view! {
                                        <CardsList cards_data=move || data.clone() render_card=|article, idx| view! { <ArticleCard article=article index=idx /> } theme=crate::components::cards_list::CardsListTheme::Light />
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
