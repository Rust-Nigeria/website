use wasm_bindgen::JsValue;
use web_sys::js_sys::{Date, Object, Reflect};

pub fn parse_iso_js(iso_str: &str) -> String {
    let val = JsValue::from_str(iso_str);
    let date = Date::new(&val);

    // Build JS object for { day: "numeric", month: "long", year: "numeric", hour: "numeric", minute: "2-digit", hour12: true }
    let options = Object::new();
    Reflect::set(
        &options,
        &JsValue::from_str("day"),
        &JsValue::from_str("numeric"),
    )
    .unwrap();
    Reflect::set(
        &options,
        &JsValue::from_str("month"),
        &JsValue::from_str("long"),
    )
    .unwrap();
    Reflect::set(
        &options,
        &JsValue::from_str("year"),
        &JsValue::from_str("numeric"),
    )
    .unwrap();
    Reflect::set(
        &options,
        &JsValue::from_str("hour"),
        &JsValue::from_str("numeric"),
    )
    .unwrap();
    Reflect::set(
        &options,
        &JsValue::from_str("minute"),
        &JsValue::from_str("2-digit"),
    )
    .unwrap();
    Reflect::set(
        &options,
        &JsValue::from_str("hour12"),
        &JsValue::from_bool(true),
    )
    .unwrap();

    // Format with locale and options
    let val: String = date.to_locale_string("en-US", &options.into()).into();

    val
}
