#[allow(unused_imports)]
use leptos::{logging::log, prelude::ServerFnError, server};

#[server(FormHandler, "/api")]
pub async fn form_handler(username: String) -> Result<String, ServerFnError> {
    log!("Received form submission for user: {:?}", username);
    // Process the form data
    Ok(username)
}