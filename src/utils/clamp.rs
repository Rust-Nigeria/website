use web_sys::js_sys::Math::{max, min};

pub fn clamp(minimum: f64, maximum: f64, value: f64) -> f64 {
    max(minimum, min(value, maximum))
}
