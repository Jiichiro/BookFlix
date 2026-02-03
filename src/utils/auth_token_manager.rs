use leptos::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct TokenStore {
    token: Arc<Mutex<Option<String>>>,
}

impl TokenStore {
    pub fn new() -> Self {
        Self {
            token: Arc::new(Mutex::new(None)),
        }
    }

    pub fn save_token(&self, token: String) {
        let mut t = self.token.lock().unwrap();
        *t = Some(token);
        println!("✓ Token saved to TokenStore");
    }

    pub fn get_token(&self) -> Option<String> {
        let t = self.token.lock().unwrap();
        t.clone()
    }

    pub fn has_token(&self) -> bool {
        let t = self.token.lock().unwrap();
        t.is_some()
    }

    pub fn clear_token(&self) {
        let mut t = self.token.lock().unwrap();
        *t = None;
    }
}

pub fn use_token_store() -> TokenStore {
    expect_context::<TokenStore>()
}

#[component]
pub fn TokenProvider(children: Children) -> impl IntoView {
    let store = TokenStore::new();
    provide_context(store);
    children()
}