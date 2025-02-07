use crate::model::{BookInsert, Books, BooksResponse, IDbooks, InsertResponse, SearchQuery};

use actix_web::{error, get, post, web, Responder, Result};
use sqlx::Row;
use sqlx::SqlitePool;

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

#[get("/find_by_id")]
pub async fn find_by_id(form: web::Form<IDbooks>) -> Result<impl Responder> {
    let db_url = String::from("sqlite://sqlite.db");

    let query = "SELECT * FROM books_db  WHERE id_book=$1";

    let instances = SqlitePool::connect(&db_url).await.unwrap();

    let buffer_exec = sqlx::query(&query)
        .bind(form.id_book)
        .fetch_all(&instances)
        .await
        .unwrap();

    instances.close().await;

    if buffer_exec.is_empty() {
        return Err(error::ErrorNotFound("Data Not Found"));
    }
    let books: Books = Books {
        id_book: buffer_exec[0].get("id_book"),
        name_book: buffer_exec[0].get("name_book"),
        author_book: buffer_exec[0].get("author_book"),
        created_on: buffer_exec[0].get("created_on"),
        updated_on: buffer_exec[0].get("updated_on"),
    };

    let response: BooksResponse = BooksResponse {
        status: String::from("Data Found"),
        results: books,
    };
    Ok(web::Json(response))
}

#[get("/find_by_name")]
pub async fn find_by_name(form: web::Form<SearchQuery>) -> Result<impl Responder> {
    let db_url = String::from("sqlite://sqlite.db");

    let query = "SELECT * FROM books_db  WHERE name_book LIKE $1";

    let instances = SqlitePool::connect(&db_url).await.unwrap();

    let like_symbol: String = "%".to_owned();
    let new_format = format!(
        "{like_symbol}{name_search}{like_symbol}",
        like_symbol = like_symbol,
        name_search = form.name_search
    );

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

    Ok(web::Json(books))
}

#[get("/find_by_author")]
pub async fn find_by_author(form: web::Form<SearchQuery>) -> Result<impl Responder> {
    let db_url = String::from("sqlite://sqlite.db");

    let query = "SELECT * FROM books_db  WHERE author_book LIKE $1";

    let instances = SqlitePool::connect(&db_url).await.unwrap();

    let like_symbol: String = "%".to_owned();
    let new_format = format!(
        "{like_symbol}{author_search}{like_symbol}",
        like_symbol = like_symbol,
        author_search = form.name_search
    );

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

    Ok(web::Json(books))
}

#[post("/add_book")]
pub async fn insert_db_api(form: web::Form<BookInsert>) -> Result<impl Responder> {
    let db_url = String::from("sqlite://sqlite.db");

    let query = "INSERT INTO books_db (name_book,author_book) VALUES($1,$2)";

    let instances = SqlitePool::connect(&db_url).await.unwrap();

    match sqlx::query(&query)
        .bind(&form.name_book)
        .bind(&form.author_book)
        .execute(&instances)
        .await
    {
        Ok(result) => {
            instances.close().await;
            if result.rows_affected() > 0 {
                let buffer_response: InsertResponse = InsertResponse {
                    status: String::from("success"),
                    message: String::from("Book inserted successfully"),
                    rows_affected: result.rows_affected(),
                };
                Ok(web::Json(buffer_response))
            } else {
                let buffer_response: InsertResponse = InsertResponse {
                    status: String::from("Error!"),
                    message: String::from("No rows were inserted"),
                    rows_affected: 0,
                };
                Ok(web::Json(buffer_response))
            }
        }

        Err(e) => {
            instances.close().await;
            let buffer_response: InsertResponse = InsertResponse {
                status: String::from("Error!"),
                message: format!("Failed to insert book: {}", e),
                rows_affected: 0,
            };
            Ok(web::Json(buffer_response))
        }
    }
}
