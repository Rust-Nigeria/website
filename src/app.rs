use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

use crate::components::{
    footer::Footer, join_dialog::JoinDialog, nav::Nav, providers::AppProviders,
};
use crate::pages::{home::HomePage, not_found::NotFound};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-nigeria-website.css"/>
        <Meta name="description" content="The Nigerian Rust community website. Find out about community projects, events and articles." />
        // sets the document title
        <Title text="Welcome to Rust Nigeria"/>

        // content for this welcome page
        <Router>
          <AppProviders>
            <Nav />
            <Routes fallback=move || "Not found.">
                <Route path=StaticSegment("") view=HomePage/>
                <Route path=WildcardSegment("any") view=NotFound/>
            </Routes>
            <Footer />
            <JoinDialog />
          </AppProviders>
        </Router>
    }
}
