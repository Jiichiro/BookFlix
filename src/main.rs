#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{Router, extract::Request, middleware::{self, Next, from_fn}};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use perpustakaan::app::*;
    use sqlx::MySqlPool;
    use dotenv::dotenv;
    use std::sync::Arc;
    

    dotenv().ok();

    let db_pool = Arc::new(
        MySqlPool::connect(std::env::var("DATABASE_URL").unwrap().as_str())
            .await
            .expect("Failed to connect to the database")
    );
    
    match sqlx::migrate!("./migrations").run(db_pool.as_ref()).await {
            Ok(_) => {
                log!("Database migrations applied successfully");
            }
            Err(e) => {
                log!("Error applying database migrations: {}", e);
            }
        }

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(middleware::from_fn(perpustakaan::auth_middleware::token_middleware))
        .layer(from_fn({
            let db_pool = db_pool.clone();
            move |mut req: Request, next: Next| {
                let db_pool = db_pool.clone();
                async move {
                    req.extensions_mut().insert(db_pool);
                    next.run(req).await
                }
            }
        }))
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