use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

use crate::components::{footer::view::Footer, nav::view::Nav};
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

        // content for this welcome page
        <Router>
            <Nav />
            <Routes fallback=move || "Not found.">
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=WildcardSegment("any") view=NotFound/>
            </Routes>
            <Footer />
        </Router>
    }
}
