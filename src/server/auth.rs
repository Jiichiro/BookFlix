use leptos::{ prelude::ServerFnError, server};

#[cfg(not(feature = "ssr"))]
use leptos::lazy;

#[cfg(feature = "ssr")]
use crate::User;
#[cfg(feature = "ssr")]
use axum::http::{HeaderValue, header::SET_COOKIE, request::Parts};
#[cfg(feature = "ssr")]
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
#[cfg(feature = "ssr")]
use bcrypt::DEFAULT_COST;
#[cfg(feature = "ssr")]
use leptos::prelude::expect_context;
#[cfg(feature = "ssr")]
use leptos_axum::{ResponseOptions, extract};
#[cfg(feature = "ssr")]
use moka::future::Cache;
#[cfg(feature = "ssr")]
use sqlx::{MySql, Pool, Row};
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use time::Duration;

#[cfg(feature = "ssr")]
fn generate_token() -> String {
    use base64::Engine;
    use rand::TryRngCore;

    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.try_fill_bytes(&mut bytes).unwrap();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
}

#[cfg(feature = "ssr")]
fn get_db_pool(parts: &Parts) -> Result<Arc<Pool<MySql>>, ServerFnError> {
    let db_pool = parts
        .extensions
        .get::<Arc<Pool<MySql>>>()
        .ok_or_else(|| ServerFnError::new("Database pool not found"))?
        .clone();
    Ok(db_pool)
}

#[cfg(feature = "ssr")]
fn get_user_cache(parts: &Parts) -> Result<Arc<Cache<String, User>>, ServerFnError> {
    let cache = parts
        .extensions
        .get::<Arc<Cache<String, User>>>()
        .ok_or_else(|| ServerFnError::new("no cache"))?
        .clone();
    Ok(cache)
}

#[server(Login, "/api")]
#[lazy]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    let parts = extract::<Parts>().await?;
    let resp = expect_context::<ResponseOptions>();

    let db_pool = get_db_pool(&parts)?;
    let user_cache = get_user_cache(&parts)?;

    let row = sqlx::query(
        "SELECT username, role, email, full_name, password_hash FROM users WHERE username = ?",
    )
    .bind(&username)
    .fetch_one(db_pool.as_ref())
    .await
    .map_err(|_| ServerFnError::new("User not found. Check your credential"))?;

    let password_hash: String = row.get("password_hash");

    let is_valid = bcrypt::verify(&password, &password_hash)
        .map_err(|_| ServerFnError::new("User not found. Check your credential"))
        .unwrap();

    if is_valid {
        let token = generate_token();
        let cookie = Cookie::build(("session", token.clone()))
            .path("/")
            .max_age(Duration::hours(24))
            .same_site(SameSite::Strict)
            .http_only(true)
            .build();
        resp.insert_header(
            SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
        sqlx::query("UPDATE users SET token = ? WHERE username = ?")
            .bind(&token)
            .bind(&username)
            .execute(db_pool.as_ref())
            .await
            .map_err(|e| {
                println!("{:?}", e);
                ServerFnError::new("Failed to update user token")
            })?;

        user_cache
            .insert(
                format!("session:{}", token),
                User {
                    name: row.get("username"),
                    role: row.get("role"),
                    email: row.get("email"),
                    fullname: row.get("full_name"),
                },
            )
            .await;
        Ok(())
    } else {
        Err(ServerFnError::new("User not found. Check your credential"))
    }
}

#[server(Register, "/api")]
#[lazy]
pub async fn register(
    username: String,
    email: String,
    password: String,
    confirm_password: String,
) -> Result<String, ServerFnError> {
    if password != confirm_password {
        return Err(ServerFnError::new("password not match"));
    }

    let parts = extract::<Parts>().await?;
    let db_pool = get_db_pool(&parts)?;
    let password_hash = bcrypt::hash(password, DEFAULT_COST)?;

    let insert_result =
        sqlx::query("INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)")
            .bind(username)
            .bind(email)
            .bind(password_hash)
            .execute(db_pool.as_ref())
            .await;

    match insert_result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok("Registration successful".to_string())
            } else {
                Err(ServerFnError::ServerError("Registration Failed".to_string()))
            }
        }
        Err(e) => {
            if e.to_string().contains("UNIQUE") || e.to_string().contains("duplicate") {
                Err(ServerFnError::ServerError(
                    "Username or email was registered".to_string(),
                ))
            } else {
                Err(ServerFnError::ServerError(format!("Database error: {}", e)))
            }
        }
    }
}

#[server(Logout, "/api")]
#[lazy]
pub async fn logout() -> Result<(), ServerFnError> {
    let parts = extract::<Parts>().await?;
    let resp = expect_context::<ResponseOptions>();

    let db_pool = get_db_pool(&parts)?;

    let jar = extract::<CookieJar>().await?;
    if let Some(cookie) = jar.get("session") {
        let token = cookie.value().to_owned().to_string();
        sqlx::query("UPDATE users SET token = NULL WHERE token = ?")
            .bind(&token)
            .execute(db_pool.as_ref())
            .await
            .map_err(|_| ServerFnError::new("Failed to clear user token"))?;
    }

    let expired_cookie = Cookie::build(("session", ""))
        .path("/")
        .max_age(Duration::seconds(0))
        .same_site(SameSite::Strict)
        .http_only(true)
        .build();
    resp.insert_header(
        SET_COOKIE,
        HeaderValue::from_str(&expired_cookie.to_string()).unwrap(),
    );

    Ok(())
}
