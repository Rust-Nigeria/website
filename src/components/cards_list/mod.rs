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
use std::fmt::Debug;

pub trait CardsListItem {
    fn get_key(&self) -> String;
}

#[component]
pub fn CardList<T, N, F, IV>(
    cards_data: T,
    render_card: F,
    #[prop(optional)] title: Option<&'static str>,
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

        if is_larger_than_xl.get() {
            count = 4;
        }

        count
    });

    let PaginationData {
        current_page,
        data: paginated_events,
        set_current_page,
        total_pages,
    } = use_pagination(cards_data, col_count);

    let page_items = move || {
        let pagination = PaginationUiData::new(total_pages.get(), current_page.get() as usize);
        pagination.items()
    };

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
        <div class="grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 h-full gap-x-4 overflow-x-hidden">
            <ForEnumerate
                each=move || paginated_events()
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
                {
                    page_items().into_iter()
                    .map(|value|
                    view! {
                        <li>
                            {move || {
                                match value {
                                    PageItem::Ellipsis => Either::Left(view! { <div class="text-grey-70">...</div> }),
                                    PageItem::Page(num) => Either::Right(view! { <button class=cn!(#(
                                        "size-10 text-lg rounded-full bg-white text-grey-20 cursor-pointer duration-300",
                                        (num != current_page.get(), "bg-transparent text-white hover:opacity-50")
                                    )) on:click=move |_| set_current_page(num)>{num}</button> })
                                }
                            }}
                        </li>
                    })
                    .collect_view()
                }
            }}
        </ul>
     </div>
    }
}
