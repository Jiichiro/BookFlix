pub mod app;
mod server;
mod pages;
mod components;
mod layouts;
mod models;

#[cfg(feature = "ssr")]
pub use {
    server::middleware::auth_middleware,
    models::User
};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_lazy(App);
}
