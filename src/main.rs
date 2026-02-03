#![recursion_limit = "512"]
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::{sync::Arc, time::Duration};

    use axum::{Extension, Router, middleware, response::Redirect, routing::get};
    use bookflix::{User, app::*, auth_middleware};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use moka::future::Cache;
    use sqlx::MySqlPool;

    dotenvy::dotenv().expect("Failed to load .env file");

    let db_pool = Arc::new(
        MySqlPool::connect(std::env::var("DATABASE_URL").unwrap().as_str())
            .await
            .expect("Failed to connect to the database"),
    );

    sqlx::migrate!("./migrations")
        .run(db_pool.as_ref())
        .await
        .expect("Failed to run database migrations");

    let user_cache = Arc::new(
        Cache::<String, User>::builder()
            .max_capacity(100_000)
            .time_to_idle(Duration::from_secs(300))
            .build(),
    );

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/", get(|| async { Redirect::temporary("/login") }))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(middleware::from_fn(auth_middleware))
        .layer(Extension(db_pool.clone()))
        .layer(Extension(user_cache.clone()))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
