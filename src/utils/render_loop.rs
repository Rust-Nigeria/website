use wasm_bindgen::prelude::Closure;

#[derive(Default)]
pub struct RenderLoop {
    pub animation_id: Option<i32>,
    pub closure: Option<Closure<dyn Fn(f64)>>,
}

impl RenderLoop {
    pub fn new(animation_id: Option<i32>, closure: Option<Closure<dyn Fn(f64)>>) -> RenderLoop {
        RenderLoop {
            animation_id,
            closure,
        }
    }
}

