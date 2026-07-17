use wasm_bindgen::prelude::Closure;

#[derive(Default)]
pub struct RenderLoop {
    pub animation_id: Option<i32>,
    pub closure: Option<Closure<dyn Fn(f64)>>,
}

impl RenderLoop {
    pub fn cancel(&self) {
        if let Some(animation_id) = self.animation_id {
            let window =
                web_sys::window().expect("Failed to get window when cleaning up animation loop");

            window
                .cancel_animation_frame(animation_id)
                .expect("Cannot Cancel Animation Frame");
        }
    }
}

impl Drop for RenderLoop {
    fn drop(&mut self) {
        self.cancel();
    }
}
