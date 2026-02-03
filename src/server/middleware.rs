use std::sync::Arc;

use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use moka::future::Cache;
use sqlx::{MySql, Pool, Row};

use crate::User;

pub async fn auth_middleware(req: Request, next: Next) -> impl IntoResponse {
    let jar = CookieJar::from_headers(req.headers());

    let cookie = jar
        .get("session")
        .map(|v| v.value().to_owned().to_string())
        .unwrap_or_default();

    let path = req.uri().path();

    let user_cache = req
        .extensions()
        .get::<Arc<Cache<String, User>>>()
        .expect("Memory cache error. Make sure your memory not full");

    let db_pool = req
        .extensions()
        .get::<Arc<Pool<MySql>>>()
        .cloned()
        .expect("Database pool not found. Call your administrator.");

    let allowed_path = [
        "/includes/",
        "/api/login",
        "/login",
        "/api/register",
        "/register",
    ];

    if allowed_path
        .iter()
        .any(|p| path == *p || path.starts_with(*p))
    {
        return next.run(req).await;
    }

    if cookie.is_empty() {
        return redirect_to_login().into_response();
    }

    if let Some(_) = user_cache.get(&format!("session:{}", cookie)).await {
        return next.run(req).await;
    }

    let count = match sqlx::query("SELECT COUNT(*) as count FROM users WHERE token = ?")
        .bind(&cookie)
        .fetch_one(db_pool.as_ref())
        .await
    {
        Ok(c) => c.get::<i32, _>("count"),
        Err(e) => {
            println!("Database query error: {}", e);
            return redirect_to_login().into_response();
        }
    };
    if count == 0 {
        return redirect_to_login().into_response();
    }

    match sqlx::query("SELECT username, email, role, full_name FROM users WHERE token = ?")
        .bind(&cookie)
        .fetch_one(db_pool.as_ref())
        .await
    {
        Ok(user) => {
            user_cache
                .insert(
                    format!("session:{}", cookie),
                    User {
                        name: user.get::<String, _>("username"),
                        role: user.get::<String, _>("role"),
                        email: user.get::<String, _>("email"),
                        fullname: user.get::<Option<String>, _>("full_name"),
                    },
                )
                .await;
        }
        Err(e) => {
            println!("cache error: {}", e);
        }
    }

    next.run(req).await
}

fn redirect_to_login() -> impl IntoResponse {
    (
        StatusCode::TEMPORARY_REDIRECT,
        [(header::LOCATION, "/login")],
    )
}
