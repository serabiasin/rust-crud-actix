mod db;
mod handlers;
mod model;
use crate::model::{Books, BooksResponse};
use actix_web::{web, App, HttpServer};

use db::*;
use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = String::from("sqlite://sqlite.db");

    check_db(&db_url).await; //menunggu proses selesai, karena async
    insert_db(&db_url, "Hello world adalah kita", "Joke 'O We").await;
    // let id: u32 = 2;
    // select_data_db(&id).await;
    HttpServer::new(|| {
        App::new()
            .service(find_by_id)
            .service(find_by_name)
            .service(find_by_author)
            .service(insert_db_api)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
