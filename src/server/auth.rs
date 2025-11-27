// src/server/auth.rs

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::models::Claims;
#[cfg(feature = "ssr")]
use axum::http::request::Parts;
#[cfg(feature = "ssr")]
use chrono::{Duration, Utc};
#[cfg(feature = "ssr")]
use jsonwebtoken::{encode, Header};
#[cfg(feature = "ssr")]
use leptos_axum::extract;
#[cfg(feature = "ssr")]
use sqlx::{MySql, Pool};
#[cfg(feature = "ssr")]
use std::sync::Arc;

// ===== Login Server Function =====
#[server(Login, "/auth")]
pub async fn login(username: String, password: String) -> Result<String, ServerFnError> {
    use jsonwebtoken::EncodingKey;
    use sqlx::Row;

    // Get database connection from request extensions
    let db = extract::<Parts>()
        .await
        .ok()
        .and_then(|req| req.extensions.get::<Arc<Pool<MySql>>>().cloned())
        .ok_or_else(|| ServerFnError::new("Database connection not available".to_string()))?;

    // Validasi input
    if username.is_empty() || password.is_empty() {
        return Err(ServerFnError::ServerError(
            "Username dan password harus diisi".to_string(),
        ));
    }

    // Cari user di database
    let user =
        sqlx::query("SELECT id, username, email, password_hash FROM users WHERE username = ?")
            .bind(&username)
            .fetch_one(db.as_ref())
            .await
            .map_err(|_| ServerFnError::new("Username atau password salah".to_string()))?;

    let user_id: i64 = user.get("id");
    let password_hash: String = user.get("password_hash");

    // Verifikasi password dengan bcrypt
    let is_valid = bcrypt::verify(&password, &password_hash)
        .map_err(|_| ServerFnError::new("Password verification failed".to_string()))?;

    if !is_valid {
        eprintln!("Failed login attempt for user {}", username);
        return Err(ServerFnError::ServerError(
            "Username atau password salah".to_string(),
        ));
    }

    // Update last_login
    sqlx::query("UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(user_id)
        .execute(db.as_ref())
        .await
        .map_err(|_| ServerFnError::new("Failed to update last login".to_string()))?;

    // Generate JWT token
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

    let now = Utc::now();
    let exp = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        username: username.clone(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| ServerFnError::new("Failed to create token".to_string()))?;

    #[cfg(debug_assertions)]
    println!("User {} logged in successfully", username);

    Ok(token)
}

// ===== Register Server Function =====
#[server(Register, "/auth")]
pub async fn register(
    username: String,
    email: String,
    password: String,
    password_confirm: String,
) -> Result<String, ServerFnError> {
    use jsonwebtoken::EncodingKey;

    // Get database connection from request extensions
    let db = extract::<Parts>()
        .await
        .ok()
        .and_then(|req| req.extensions.get::<Arc<Pool<MySql>>>().cloned())
        .ok_or_else(|| {
            ServerFnError::new("Database connection not available".to_string())
        })?;

    // Validasi input
    if username.is_empty() || email.is_empty() || password.is_empty() {
        return Err(ServerFnError::ServerError(
            "Semua field harus diisi".to_string(),
        ));
    }

    if password != password_confirm {
        return Err(ServerFnError::ServerError(
            "Password tidak cocok".to_string(),
        ));
    }

    if password.len() < 6 {
        return Err(ServerFnError::ServerError(
            "Password minimal 6 karakter".to_string(),
        ));
    }

    // Cek apakah user sudah terdaftar
    let existing_user = sqlx::query("SELECT id FROM users WHERE username = ? OR email = ?")
        .bind(&username)
        .bind(&email)
        .fetch_optional(db.as_ref())
        .await
        .map_err(|_| ServerFnError::new("Database query failed".to_string()))?;

    if existing_user.is_some() {
        return Err(ServerFnError::ServerError(
            "Username atau email sudah terdaftar".to_string(),
        ));
    }

    // Hash password
    let password_hash = bcrypt::hash(&password, 4)
        .map_err(|_| ServerFnError::new("Failed to hash password".to_string()))?;

    // Insert user ke database
    let result = sqlx::query("INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)")
        .bind(&username)
        .bind(&email)
        .bind(&password_hash)
        .execute(db.as_ref())
        .await
        .map_err(|_| ServerFnError::new("Failed to create user".to_string()))?;

    let user_id = result.last_insert_id();

    // Generate JWT token
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

    let now = Utc::now();
    let exp = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id as i64,
        username: username.clone(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| ServerFnError::new("Failed to create token".to_string()))?;

    #[cfg(debug_assertions)]
    println!("New user {} registered successfully", username);

    Ok(token)
}

// ===== Validate Token Server Function =====
#[server(ValidateToken, "/auth")]
pub async fn validate_token(token: String) -> Result<String, ServerFnError> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| ServerFnError::new("Invalid token".to_string()))?;

    Ok(claims.username)
}
