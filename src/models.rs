use leptos::config::LeptosOptions;
use sqlx::PgPool;

pub struct AppState {
    pub db_pool: PgPool,
    pub leptos_options: LeptosOptions,
}