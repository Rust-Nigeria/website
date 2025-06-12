use leptos::{ev, prelude::*};

use crate::cn;

#[component]
pub fn RevealTextLine(
    children: Children,
    reveal: bool,
    #[prop(default = "")] class: &'static str,
    #[prop(default = "")] parent_class: &'static str,
    #[prop(default = false)] rotate: bool,
    #[prop(default = 0)] delay: u16,
) -> impl IntoView {
    view! {
        <span class=cn!(#("overflow-y-hidden inline-flex", parent_class))>
            <span class=cn!(#(
                cn!("translate-y-full inline-flex opacity-0 duration-500", (rotate, "rotate-6 origin-top-left")),
                (reveal, "translate-y-0 opacity-100", (rotate, "rotate-0")),
                class
            ))>
               {children()}
            </span>
        </span>
    }
}
