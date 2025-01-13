use std::cell::Cell;
use std::{cell::RefCell, rc::Rc};

use leptos::{html, prelude::*};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::js_sys::Date;

use crate::utils::clamp::clamp;
use crate::utils::render_loop::RenderLoop;

use super::jigsaw::Jigsaw;

const REVEAL_DURATION: f64 = 5000.0;

pub fn start_animation_loop(jigsaw: Jigsaw, duration: RwSignal<f64>) {
    let render_loop: Rc<RefCell<RenderLoop>> = Rc::new(RefCell::new(RenderLoop::new(None, None)));
    let window = web_sys::window().unwrap();
    let time = Cell::new(Date::now());

    let closure: Closure<dyn Fn(f64)> = {
        let window = web_sys::window().unwrap();
        let render_loop = render_loop.clone();
        Closure::wrap(Box::new(move |_| {
            let now = Date::now();
            let dt = now - time.get();
            time.replace(now);
            duration.set(clamp(0.0, REVEAL_DURATION, duration.get_untracked() + dt));
            jigsaw.render(duration.get_untracked());
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

    let duration = RwSignal::new(0.0 as f64);

    let maybe_jigsaw = LocalResource::new(move || Jigsaw::new(canvas_ref, REVEAL_DURATION));

    Effect::new(move || {
        if let Some(jigsaw) = maybe_jigsaw.get() {
            start_animation_loop(jigsaw.take(), duration);
        }
    });

    view! {
        <canvas node_ref={canvas_ref} class="aspect-[857/571] w-full"></canvas>
    }
}
