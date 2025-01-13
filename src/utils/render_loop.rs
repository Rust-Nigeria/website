use wasm_bindgen::prelude::Closure;

pub struct RenderLoop {
    pub animation_id: Option<i32>,
    pub closure: Option<Closure<dyn Fn()>>,
}

impl RenderLoop {
    pub fn new(animation_id: Option<i32>, closure: Option<Closure<dyn Fn()>>) -> RenderLoop {
        RenderLoop {
            animation_id,
            closure,
        }
    }
}
