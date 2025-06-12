use leptos::prelude::window;
use web_sys::HtmlCanvasElement;

pub fn resize_canvas_to_display_size(canvas: &HtmlCanvasElement) {
    let dpr = window().device_pixel_ratio() as i32;

    let canvas_width = canvas.width() as i32;
    let canvas_height = canvas.height() as i32;

    let client_width = canvas.client_width();
    let client_height = canvas.client_height();

    let display_width = client_width * dpr;
    let display_height = client_height * dpr;

    let need_resize = canvas_width != display_width || canvas_height != display_height;

    if need_resize {
        canvas.set_width(display_width as u32);
        canvas.set_height(display_height as u32);
    }
}
