#[cfg(feature = "ssr")]
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

#[cfg(feature = "ssr")]
pub async fn token_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    // ✅ BYPASS untuk login/register
    let path = req.uri().path().to_string();
    if path.starts_with("/login") || path.starts_with("/register") {
        return next.run(req).await;
    }

    // ✅ AMBIL HEADER "Authorization: Bearer token"
    let auth_owned = req
        .headers()
        .get("Authorization")
        .and_then(|hv| hv.to_str().ok().map(|s| s.to_string()));

    if let Some(auth_str) = auth_owned {
        println!("✓ Authorization header found: {}", auth_str);
        
        // ✅ EXTRACT TOKEN DARI FORMAT "Bearer token123"
        if let Some(token) = auth_str.strip_prefix("Bearer ") {
            println!("✓ Token extracted: {}", token);
            
            // ✅ SIMPAN TOKEN DI REQUEST.EXTENSIONS
            req.extensions_mut().insert(token.to_string());
            println!("✓ Token saved to req.extensions");
        }
    } else {
        println!("⚠ No Authorization header found");
    }

    next.run(req).await
}