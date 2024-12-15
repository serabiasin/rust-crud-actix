use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Books {
    pub id_book: u32,
    pub name_book: String,
    pub author_book: String,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct IDbooks {
    pub id_book: u32,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub name_search: String,
}
#[derive(Deserialize)]
pub struct BookInsert {
    pub name_book: String,
    pub author_book: String,
}

#[derive(Serialize, Debug)]
pub struct BooksResponse {
    pub status: String,
    pub results: Books,
}

#[derive(Serialize, Debug)]
pub struct InsertResponse {
    pub status: String,
    pub message: String,
    pub rows_affected: u64,
}
