use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::global_components::{body::view::Body, nav::view::Nav};
use crate::pages::{home::view::HomePage, not_found::view::NotFound};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-nigeria-website.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Router>
           <Body>
                <Nav />
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
           </Body>
        </Router>
    }
}
