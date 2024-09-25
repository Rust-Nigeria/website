use leptos::*;

#[component]
pub fn Body(children: Children) -> impl IntoView {
    view! {
       <body class="relative flex flex-col w-full items-center">
        {children()}
       </body>
    }
}
