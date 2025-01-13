use std::{cell::RefCell, rc::Rc};

use leptos::{html, prelude::*};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use crate::utils::render_loop::RenderLoop;

use super::jigsaw::Jigsaw;

pub fn start_animation_loop(jigsaw: Jigsaw) {
    let render_loop: Rc<RefCell<RenderLoop>> = Rc::new(RefCell::new(RenderLoop::new(None, None)));
    let window = web_sys::window().unwrap();

    let closure: Closure<dyn Fn()> = {
        let window = web_sys::window().unwrap();
        let render_loop = render_loop.clone();
        Closure::wrap(Box::new(move || {
            jigsaw.render();
            let mut render_loop = render_loop.borrow_mut();
            render_loop.animation_id = if let Some(ref closure) = render_loop.closure {
                Some(
                    window
                        .request_animation_frame(closure.as_ref().unchecked_ref())
                        .expect("cannot set animation frame"),
                )
            } else {
                None
            }
        }))
    };

    let mut render_loop = render_loop.borrow_mut();

    render_loop.animation_id = Some(
        window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .expect("cannot set animation frame"),
    );
    render_loop.closure = Some(closure);
}

#[component]
pub fn MembersJigsaw() -> impl IntoView {
    let canvas_ref: NodeRef<html::Canvas> = NodeRef::new();

    let maybe_jigsaw = LocalResource::new(move || Jigsaw::new(canvas_ref));

    Effect::new(move || {
        if let Some(jigsaw) = maybe_jigsaw.get() {
            start_animation_loop(jigsaw.take());
        }
    });

    view! {
        <canvas node_ref={canvas_ref} class="aspect-[857/571] w-full"></canvas>
    }
}
