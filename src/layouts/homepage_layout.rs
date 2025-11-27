use leptos::{*, html::*, prelude::*};
use leptos_router::components::Outlet;

use crate::components::{Footer, NavBar};

#[component]
#[allow(non_snake_case)]
pub fn HomepageLayout() -> impl IntoView {
    view! {
        <NavBar />
            <main class="flex-grow">
                <Outlet />
            </main>
        <Footer />
    }
}