use leptos::{html::ElementType, prelude::*};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::IntersectionObserverInit;

use crate::utils::intersection_observer_wrap::IntersectionObserverWrap;

pub struct ElementVisibilityData {
    pub in_view: ReadSignal<bool>,
}

pub fn use_in_view<T>(
    element: NodeRef<T>,
    options: Option<IntersectionObserverInit>,
) -> ElementVisibilityData
where
    T: ElementType,
    T::Output: AsRef<web_sys::Element> + Clone + 'static + JsCast,
{
    let (in_view, set_in_view) = signal(false);
    let element_observer_wrap: Rc<RefCell<Option<IntersectionObserverWrap>>> =
        Rc::new(RefCell::new(None));
    let observer_clone = element_observer_wrap.clone();

    Effect::new(move || {
        if observer_clone.borrow().is_none() {
            let set_visible = set_in_view.clone();
            let observer = IntersectionObserverWrap::new(
                Box::new(move |entries, _| {
                    set_visible(entries[0].is_intersecting());
                }),
                options.as_ref(),
            );
            *observer_clone.borrow_mut() = Some(observer);
        }

        if let Some(el) = element.get() {
            if let Some(observer) = &*observer_clone.borrow() {
                observer.get().observe(&el.as_ref());
            }
        }
    });

    ElementVisibilityData { in_view }
}
