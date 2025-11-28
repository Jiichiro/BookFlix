pub mod app;
mod server;
mod pages;
mod components;
mod layouts;
mod models;
mod utils;

#[cfg(feature = "ssr")]
pub use crate::server::auth_middleware;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
