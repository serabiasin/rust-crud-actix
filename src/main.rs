mod db;
use db::{*};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");

    check_db(&db_url).await; //menunggu proses selesai, karena async
    insert_db(&db_url, "Mulyono adalah kita", "Joke 'O We").await;
    let id:u32=2;
    select_data_db(&db_url, &id).await;
}
