use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    Lazy,
    components::{ParentRoute, Route, Router, Routes},
    path,
};

use crate::layouts::{AdminLayout, HomepageLayout};
use crate::pages::{
    LoginPage, NotFound, RegisterPage,
    admin::AdminDashboardPage,
    member::{BookDetails, HomePage},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="reading a book"/>
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
        <Stylesheet id="leptos" href="/pkg/bookflix.css"/>
        // content for this welcome page
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/login") view={Lazy::<LoginPage>::new()}/>
                <Route path=path!("/register") view={Lazy::<RegisterPage>::new()}/>
                <ParentRoute path=path!("/member") view=HomepageLayout>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("/book") view=BookDetails/>
                </ParentRoute>
                <ParentRoute path=path!("/admin") view={Lazy::<AdminLayout>::new()}>
                    <Route path=path!("") view=AdminDashboardPage/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
