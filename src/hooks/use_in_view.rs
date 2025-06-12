use leptos::{html::ElementType, prelude::*};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::IntersectionObserverInit;

use crate::utils::intersection_observer_wrap::IntersectionObserverWrap;

pub struct ElementVisibilityData {
    pub in_view: ReadSignal<bool>,
}

pub struct InViewOptions {
    pub observer_options: Option<IntersectionObserverInit>,
    pub trigger_once: Option<bool>,
}

impl Default for InViewOptions {
    fn default() -> Self {
        InViewOptions {
            observer_options: None,
            trigger_once: Some(true),
        }
    }
}

pub fn use_in_view<T>(element: NodeRef<T>, options: Option<InViewOptions>) -> ElementVisibilityData
where
    T: ElementType,
    T::Output: AsRef<web_sys::Element> + Clone + 'static + JsCast,
{
    let (in_view, set_in_view) = signal(false);

    let element_observer_wrap: Rc<RefCell<Option<IntersectionObserverWrap>>> =
        Rc::new(RefCell::new(None));

    let observer_clone_1 = element_observer_wrap.clone();
    let observer_clone_2 = element_observer_wrap.clone();

    let in_view_options = options.unwrap_or_default();

    let trigger_once = in_view_options.trigger_once.unwrap_or(false);

    Effect::new(move || {
        if observer_clone_1.borrow().is_none() {
            let set_visible = set_in_view.clone();
            let observer = IntersectionObserverWrap::new(
                Box::new(move |entries, _| {
                    set_visible(entries[0].is_intersecting());
                }),
                in_view_options.observer_options.as_ref(),
            );
            *observer_clone_1.borrow_mut() = Some(observer);
        }

        if let Some(el) = element.get() {
            if let Some(observer) = &*observer_clone_1.borrow() {
                observer.get().observe(&el.as_ref());
            }
        }
    });

    Effect::new(move || {
        if in_view() && trigger_once {
            if let Some(el) = element.get() {
                if let Some(observer) = &*observer_clone_2.borrow() {
                    observer.get().unobserve(&el.as_ref());
                }
            }
        }
    });

    ElementVisibilityData { in_view }
}
