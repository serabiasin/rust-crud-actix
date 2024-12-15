// use crate::Pool;
// use actix_web::http::StatusCode;
// use actix_web::{delete, get, patch, post, web, Responder};
// use actix_web::{Error, HttpResponse};

use sqlx::Row;
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};

pub async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;
    let qry = "
    CREATE TABLE IF NOT EXISTS books_db(
            id_book                   INTEGER PRIMARY KEY AUTOINCREMENT,
            name_book                 TEXT NOT NULL,
            author_book               TEXT NOT NULL,
            created_on                DATETIME DEFAULT (datetime('now','localtime')),
            updated_on                DATETIME DEFAULT (datetime('now','localtime'))
        );";

    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;

    return result;
}

pub async fn check_db(db_url: &str) {
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => println!("Database created Sucessfully"),
            Err(e) => panic!("{}", e),
        }
    }
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO books_db (name_book,author_book) VALUES($1,$2)";
    let result = sqlx::query(&qry)
        .bind("testing")
        .bind("test 2")
        .execute(&instances)
        .await;

    instances.close().await;

    println!("{:?}", result);
}
