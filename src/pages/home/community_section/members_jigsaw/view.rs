use leptos::{html, prelude::*};

use super::jigsaw::Jigsaw;

#[component]
pub fn MembersJigsaw() -> impl IntoView {
    let canvas_ref: NodeRef<html::Canvas> = NodeRef::new();

    let maybe_jigsaw = LocalResource::new(move || Jigsaw::new(canvas_ref));

    Effect::new(move || {
        if let Some(jigsaw) = maybe_jigsaw.get() {
            jigsaw.render();
        }
    });

    view! {
        <canvas node_ref={canvas_ref} class="aspect-[857/571] w-full"></canvas>
    }
}
