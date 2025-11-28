use crate::icons::check::Check;
use crate::utils::get_color_pair::get_color_pair;
use crate::{
    cn,
    components::{
        button::{Button, ButtonColorVariants, ButtonSizeVariants, ButtonUsecase},
        reveal_text_line::RevealTextLine,
    },
};
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

#[derive(Clone, Debug, PartialEq)]
pub struct CardListColsByBreakpoint {
    pub base: u8,
    pub sm: Option<u8>,
    pub md: Option<u8>,
    pub lg: Option<u8>,
    pub xl: Option<u8>,
}

impl Default for CardListColsByBreakpoint {
    fn default() -> Self {
        CardListColsByBreakpoint {
            sm: Some(2),
            md: None,
            lg: Some(3),
            xl: Some(4),
            base: 1,
        }
    }
}

impl CardListColsByBreakpoint {
    pub fn set_base(mut self, value: u8) -> Self {
        self.base = value;
        self
    }
    pub fn set_sm(mut self, value: u8) -> Self {
        self.sm = Some(value);
        self
    }
    pub fn set_md(mut self, value: u8) -> Self {
        self.md = Some(value);
        self
    }
    pub fn set_lg(mut self, value: u8) -> Self {
        self.lg = Some(value);
        self
    }
    pub fn set_xl(mut self, value: u8) -> Self {
        self.xl = Some(value);
        self
    }
}

#[component]
pub fn CardsList<T, N, F, IV>(
    cards_data: T,
    render_card: F,
    #[prop(optional)] title: Option<&'static str>,
    #[prop(default = CardsListTheme::Dark)] theme: CardsListTheme,
    #[prop(optional)] cols_by_breakpoint: Option<CardListColsByBreakpoint>,
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

    let is_sm = use_media_query("(min-width: 40rem)");

    let is_md = use_media_query("(min-width: 48rem)");

    let is_lg = use_media_query("(min-width: 64rem)");

    let is_xl = use_media_query("(min-width: 80rem)");

    let count_data = Memo::new(move |_| {
        let config = cols_by_breakpoint
            .clone()
            .unwrap_or(CardListColsByBreakpoint::default());

        let mut count = config.base;
        let mut multiplier = 4;

        if is_sm.get() {
            if let Some(v) = config.sm {
                count = v;
            }
        }

        if is_md.get() {
            if let Some(v) = config.md {
                count = v;
                multiplier = 2;
            }
        }

        if is_lg.get() {
            if let Some(v) = config.lg {
                count = v;
            }
        }

        if is_xl.get() {
            if let Some(v) = config.xl {
                count = v;
            }
        }

        (count, (count * multiplier) as usize)
    });

    let (items_count, set_items_count) = signal(count_data().1);

    let PaginationData {
        current_page,
        data: paginated_data,
        set_current_page,
        total_pages,
    } = use_pagination(filtered_data, items_count);

    let page_items = move || {
        let pagination = PaginationUiData::new(total_pages.get(), current_page.get());
        pagination.items()
    };

    let is_light_theme = matches!(theme, CardsListTheme::Light);

    let has_more_to_show = move || items_count() < filtered_data().len();

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

        <div
            style=move || format!("grid-template-columns: repeat({}, 1fr)", count_data().0)
         class="grid h-full gap-x-4 overflow-x-hidden">
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

        <div class=cn!(#(
            "flex w-full justify-start mt-4 md:hidden duration-300",
            (!has_more_to_show(), "opacity-70 pointer-events-none")
        ))>
            <Button
                use_as=ButtonUsecase::Button {
                on_click: Box::new(
                    move |_| {
                        set_items_count(items_count() + count_data().1)
                    }
                )
            }
                color=if is_light_theme { ButtonColorVariants::Grey } else { ButtonColorVariants::White }
                size=ButtonSizeVariants::Md
            >
                Show More
            </Button>
        </div>

        <ul class="flex items-center justify-center mt-4">
            {move || {
                let is_empty = page_items().is_empty();

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
