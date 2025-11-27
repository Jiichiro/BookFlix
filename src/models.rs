#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Books {
    pub title: String,
    pub creator: String,
    pub image: String
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
}