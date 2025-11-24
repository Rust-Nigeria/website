use crate::icons::check::Check;
use crate::utils::get_color_pair::get_color_pair;
use crate::{cn, components::reveal_text_line::RevealTextLine};
use crate::{
    hooks::{
        use_in_view::{use_in_view, ElementVisibilityData},
        use_pagination::{use_pagination, PaginationData},
    },
    utils::get_pagination_page_list::{PageItem, PaginationUiData},
};
use leptos::either::Either;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_media_query;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait CardsListItem: Send + Sync + Clone + PartialEq + Debug + 'static {
    type Tag: Display + Clone + Eq + Hash + Sync + Send + Debug + 'static;

    fn get_key(&self) -> String;
    fn get_tags(&self) -> &Vec<Self::Tag>;
}

pub enum CardsListTheme {
    Dark,
    Light,
}

#[derive(Clone, Copy)]
pub enum CardsListMaxColCount {
    Four,
    Three,
}

#[component]
pub fn CardsList<T, N, F, IV>(
    cards_data: T,
    render_card: F,
    #[prop(optional)] title: Option<&'static str>,
    #[prop(default = CardsListTheme::Dark)] theme: CardsListTheme,
    #[prop(default = CardsListMaxColCount::Four)] max_col_count: CardsListMaxColCount,
) -> impl IntoView
where
    N: CardsListItem + Send + Sync + PartialEq + Clone + Debug + 'static,
    T: Fn() -> Vec<N> + Send + Sync + 'static,
    F: Fn(N, usize) -> IV + Clone + Send + 'static,
    IV: IntoView + 'static,
{
    let section_ref = NodeRef::<Div>::new();
    let ElementVisibilityData {
        in_view: section_in_view,
    } = use_in_view(section_ref, None);

    let cards_data_memo = Memo::new(move |_| cards_data());

    let tags = move || {
        let mut tags_set = HashSet::new();
        for card_item in cards_data_memo().iter() {
            let tags = card_item.get_tags();

            for tag in tags.iter() {
                tags_set.insert(tag.clone());
            }
        }

        tags_set
    };

    let selected_tags: RwSignal<HashSet<N::Tag>> = RwSignal::new(tags());

    let filtered_data = move || {
        cards_data_memo()
            .iter()
            .filter(|d| {
                let tags = d.get_tags();
                tags.iter().any(|tag| selected_tags.read().contains(tag))
            })
            .cloned()
            .collect::<Vec<N>>()
    };

    let is_larger_than_sm = use_media_query("(min-width: 40rem)");

    let is_larger_than_lg = use_media_query("(min-width: 64rem)");

    let is_larger_than_xl = use_media_query("(min-width: 80rem)");

    let col_count = Memo::new(move |_| {
        let mut count: usize = 1;

        if is_larger_than_sm.get() {
            count = 2;
        }

        if is_larger_than_lg.get() {
            count = 3;
        }

        if matches!(max_col_count, CardsListMaxColCount::Four) && is_larger_than_xl.get() {
            count = 4;
        }

        count
    });

    let PaginationData {
        current_page,
        data: paginated_data,
        set_current_page,
        total_pages,
    } = use_pagination(filtered_data, col_count);

    let page_items = move || {
        let pagination = PaginationUiData::new(total_pages.get(), current_page.get());
        pagination.items()
    };

    let is_light_theme = matches!(theme, CardsListTheme::Light);

    Effect::new(move || selected_tags.set(tags()));

    view! {
     <div node_ref=section_ref>
        <Show when=move || title.is_some()>
            <div class="flex items-center w-full gap-x-4 mb-4">
                <h3 class="header-6">
                    <RevealTextLine reveal=section_in_view>
                        {title}
                    </RevealTextLine>
                </h3>
                <div class=cn!(#(
                    "grow h-px bg-linear-to-r from-grey-50 to-transparent scale-x-0 duration-700 origin-left",
                    (section_in_view(), "scale-x-100")
                )) />
            </div>
        </Show>
        <div class="flex mt-3 items-center gap-x-2 flex-wrap">
            {move ||
                tags().iter().enumerate()
                .map(|(index, tag)| {
                    let tag_clone = tag.clone();
                    let tag_clone_2 = tag.clone();
                    let tag_clone_3 = tag.clone();
                    let (bg, text) = get_color_pair(index, 30);
                    view! {
                        <button
                            style=format!("background-color: {}; color: {}; border-color: {}", bg, text, text)
                            class=cn!(#(
                                "text-base cursor-pointer border flex items-center rounded-full px-2 py-0.5 font-medium opacity-70",
                                (selected_tags.get().contains(&tag_clone_3), "opacity-100")
                            ))
                            on:click=move |_| {
                                let t = tag_clone.clone();
                                selected_tags.update(|set| {
                                    if set.contains(&t) {
                                        set.remove(&t);
                                    } else {
                                        set.insert(t);
                                    }
                                });
                            }
                        >
                            {format!("{}", tag)}
                            <div class=cn!(#(
                                "duration-300 overflow-hidden w-5 flex justify-end",
                                (!selected_tags.read().contains(&tag_clone_2), "w-0")
                            ))>
                                <Check {..} class="max-w-3" />
                            </div>
                        </button>
                    }
                }).collect_view()
            }
        </div>

        <div class=cn!(#(
            "grid sm:grid-cols-2 lg:grid-cols-3 h-full gap-x-4 overflow-x-hidden",
            (matches!(max_col_count, CardsListMaxColCount::Four), "xl:grid-cols-4")
        ))>
            <ForEnumerate
                each=move || paginated_data()
                key=move |ev| format!("{}-{}", ev.get_key().clone(), current_page.get())
                let(idx, ev)
            >
                <div style=format!("animation-delay: {}s", (idx.get() as f32) * 0.1) class=cn!(#("opacity-0 pt-6", (section_in_view(), "animate-fade-in-40-from-r")))>
                    {render_card(ev, idx.get())}
                </div>
            </ForEnumerate>
        </div>

        <ul class="flex items-center justify-center mt-4">
            {move || {
                let is_empty = page_items().len() == 0;

                if is_empty {
                    Either::Left(
                        view! { <div class="h-[400px] items-center justify-center flex p-5">No Data Available for this Selection</div>}
                    )
                } else {
                    Either::Right(
                        page_items().into_iter()
                        .map(|value|
                        view! {
                            <li>
                                {move || {
                                    match value {
                                        PageItem::Ellipsis => Either::Left(view! { <div class=cn!(#(
                                            "text-grey-70",
                                            (is_light_theme, "text-grey-20")
                                        ))>...</div> }),
                                        PageItem::Page(num) => Either::Right(view! { <button class=cn!(#(
                                            "size-10 sm:text-lg rounded-full bg-white text-grey-20 cursor-pointer duration-300",
                                            (is_light_theme, "bg-grey-20 text-white"),
                                            (num != current_page.get(), "bg-transparent hover:opacity-50", (!is_light_theme, "text-white"), (is_light_theme, "text-grey-20"))

                                        )) on:click=move |_| set_current_page(num)>{num}</button> })
                                    }
                                }}
                            </li>
                        }).collect_view()
                    )
                }
            }}
        </ul>
     </div>
    }
}
