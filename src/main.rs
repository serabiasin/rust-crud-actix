mod db;
use crate::db::create_schema;
use db::check_db;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");

    check_db(&db_url).await; //menunggu proses selesai, karena async
}
