use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

pub struct IntersectionObserverWrap {
    observer: IntersectionObserver,
    _callback: Closure<dyn FnMut(Vec<IntersectionObserverEntry>, IntersectionObserver)>,
}

impl IntersectionObserverWrap {
    pub fn new(
        callback: Box<dyn FnMut(Vec<IntersectionObserverEntry>, web_sys::IntersectionObserver)>,
        options: Option<&IntersectionObserverInit>,
    ) -> Self {
        let closure_callback = Closure::wrap(callback);

        let observer = match options {
            Some(opts) => IntersectionObserver::new_with_options(
                closure_callback.as_ref().unchecked_ref(),
                opts,
            ),
            None => IntersectionObserver::new(closure_callback.as_ref().unchecked_ref()),
        }
        .expect("Failed to create IntersectionObserver");

        IntersectionObserverWrap {
            observer,
            _callback: closure_callback,
        }
    }

    pub fn get(&self) -> &IntersectionObserver {
        &self.observer
    }

    pub fn disconnect(&self) {
        self.observer.disconnect();
    }
}
impl Drop for IntersectionObserverWrap {
    fn drop(&mut self) {
        self.disconnect();
    }
}
