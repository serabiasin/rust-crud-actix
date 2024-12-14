use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Books {
    pub id_book: u32,
    pub name_book: String,
    pub author_book: String,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct BooksResponse {
    pub status: String,
    pub results: Books,
}
