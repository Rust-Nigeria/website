use crate::{
    cn,
    components::button::{
        Button, ButtonColorVariants, ButtonIconTypes, ButtonSizeVariants, ButtonUsecase,
    },
    types::articles::CommunityArticle,
    utils::parse_iso_js::parse_iso_js,
};
use leptos::prelude::*;

#[component]
#[allow(unused_variables)]
pub fn ArticleCard(
    article: CommunityArticle,
    #[prop(default = "")] class: &'static str,
    #[prop(default = 0)] index: usize,
) -> impl IntoView {
    let date = RwSignal::new(None);

    Effect::new(move || {
        date.set(Some(parse_iso_js(&article.date.to_rfc3339())));
    });

    view! {
        <div class=cn!("h-full relative fle overflow-hidden flex-col w-full border-2 border-grey-90 rounded-2xl text-left", class)>

            <img src=article.banner alt=article.name.clone() class="h-[213px] object-cover overflow-hidden" />

            <div class="p-4">
                <p class="text-sm py-1 font-medium rounded-lg text-primary-50">{date}</p>

                <h3 class="mt-2 title-1 line-clamp-2 h-14">{article.name.clone()}</h3>

                <p class="text-grey-40 text-sm mt-1 line-clamp-3">{article.description}</p>

                <div class="flex items-center gap-x-2 mt-2">
                    <span class="font-semibold">AUTHORS:</span>
                    <div class="flex">
                        {article.authors.into_iter().enumerate().map(|(index, author)| view! {
                            <img style=format!("transform: translateX(-{}%)", index * 30) class="size-8 border-2 border-white rounded-full" src=author.image />
                        }).collect_view()}
                    </div>
                </div>

                <Button
                    class="mt-4 w-fit"
                    use_as=ButtonUsecase::Link { href: article.article_link }
                    color=ButtonColorVariants::Transparent
                    size=ButtonSizeVariants::Thin
                    icon={ButtonIconTypes::RightArrow}
                >
                    Learn More
                </Button>
            </div>
        </div>
    }
}
