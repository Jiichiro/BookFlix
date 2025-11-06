use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes, ParentRoute},
    path
};

use crate::{components::not_found::NotFound, pages::homepage::HomePage};
use crate::layouts::homepage_layout::HomepageLayout;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/perpustakaan.css"/>
        // content for this welcome page
        <Router>
            <Routes fallback=NotFound>
                <ParentRoute path=path!("/") view=HomepageLayout>
                    <Route path=path!("") view=HomePage/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
