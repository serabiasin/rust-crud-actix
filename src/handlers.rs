use crate::model::{Books, BooksResponse};

use sqlx::Row;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool};

pub async fn insert_db(db_url: &str, name_book: &str, author_book: &str) {
    let query = "INSERT INTO books_db (name_book,author_book) VALUES($1,$2)";

    let instances = SqlitePool::connect(&db_url).await.unwrap();

    let execute_query = sqlx::query(&query)
        .bind(name_book)
        .bind(author_book)
        .execute(&instances)
        .await;

    instances.close().await;
    print!("{:?}", execute_query);
}

pub async fn find_by_id(db_url: &str, id_books: &u32) -> Vec<Books> {
    let query = "SELECT * FROM books_db  WHERE id_book=$1";

    let instances = SqlitePool::connect(&db_url).await.unwrap();

    let buffer_exec = sqlx::query(&query)
        .bind(id_books)
        .fetch_all(&instances)
        .await
        .unwrap();

    instances.close().await;

    let books: Vec<Books> = buffer_exec
        .iter()
        .map(|row| Books {
            id_book: row.get("id_book"),
            name_book: row.get("name_book"),
            author_book: row.get("author_book"),
            created_on: row.get("created_on"),
            updated_on: row.get("updated_on"),
        })
        .collect(); // langsung membungkus struct menjadi vector

    return books;
}

pub async fn find_by_name(db_url: &str, name_search: &String) -> Vec<Books> {
    let query = "SELECT * FROM books_db  WHERE name_book LIKE $1";

    let instances = SqlitePool::connect(&db_url).await.unwrap();

    let like_symbol: String = "%".to_owned();
    let new_format = format!("{like_symbol}{name_search}{like_symbol}");

    let buffer_exec = sqlx::query(&query)
        .bind(new_format)
        .fetch_all(&instances)
        .await
        .unwrap();

    instances.close().await;

    let books: Vec<Books> = buffer_exec
        .iter()
        .map(|row| Books {
            id_book: row.get("id_book"),
            name_book: row.get("name_book"),
            author_book: row.get("author_book"),
            created_on: row.get("created_on"),
            updated_on: row.get("updated_on"),
        })
        .collect(); // langsung membungkus struct menjadi vector

    return books;
}

pub async fn find_by_author(db_url: &str, author_search: &String) -> Vec<Books> {
    let query = "SELECT * FROM books_db  WHERE author_book LIKE $1";

    let instances = SqlitePool::connect(&db_url).await.unwrap();

    let like_symbol: String = "%".to_owned();
    let new_format = format!("{like_symbol}{author_search}{like_symbol}");

    let buffer_exec = sqlx::query(&query)
        .bind(new_format)
        .fetch_all(&instances)
        .await
        .unwrap();

    instances.close().await;

    let books: Vec<Books> = buffer_exec
        .iter()
        .map(|row| Books {
            id_book: row.get("id_book"),
            name_book: row.get("name_book"),
            author_book: row.get("author_book"),
            created_on: row.get("created_on"),
            updated_on: row.get("updated_on"),
        })
        .collect(); // langsung membungkus struct menjadi vector

    return books;
}
