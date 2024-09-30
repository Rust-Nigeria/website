use leptos::*;

/// Renders the home page of your application.
use crate::components::community::Community;

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <section class="pt-10 h-screen bg-background-main min-h-fit">
            <nav class="max-w-6xl w-full">
                <ul>
                    <li>Hello World</li>
                    <Community/>
                </ul>
            </nav>
        </section>
    }
}
