#[cfg(feature = "ssr")]
use sqlx::prelude::FromRow;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Books {
    pub title: String,
    pub creator: String,
    pub image: String
}

#[cfg(feature = "ssr")]
#[derive(Clone, FromRow)]
pub struct User {
    pub name: String,
    pub role: String,
    pub email: String,
    pub fullname: Option<String>
}
