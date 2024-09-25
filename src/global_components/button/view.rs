use leptos::*;

#[component]
pub fn Button() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
       <button>Join Us</button>
    }
}
