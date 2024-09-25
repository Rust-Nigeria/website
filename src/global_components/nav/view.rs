use leptos::*;

use crate::global_components::button::view::Button;

#[component]
pub fn Nav() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <nav class="max-w-6xl absolute top-10 w-full mx-auto">
            <ul>
                <li>
                    <Button />
                </li>
            </ul>
        </nav>
    }
}
