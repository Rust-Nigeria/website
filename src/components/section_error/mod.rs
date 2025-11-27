use leptos::prelude::*;

#[component]
pub fn SectionError(children: Children) -> impl IntoView {
    view! {
        <div class="w-full p-4 mb-6 rounded-lg bg-red-600/20 border border-red-600 text-red-500">
            <p class="font-semibold">{children()}</p>
        </div>
    }
}
