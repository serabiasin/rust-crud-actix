use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};
use sqlx:: Row;

// #[derive(Debug, FromRow)]

// struct Books{
//     id_book:i32,
//     name_book:String,
//     author_book:String,
//     created_on:String,
//     updated_on:String
// }

pub async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;
    let qry =
    "
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

pub async fn check_db(db_url:&str){
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => println!("Database created Sucessfully"),
            Err(e) => panic!("{}", e),
        }
    }
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO books_db (name_book,author_book) VALUES($1,$2)";
    let result = sqlx::query(&qry).bind("testing").bind("test 2").execute(&instances).await;

    instances.close().await;

    println!("{:?}", result);
}



pub async fn insert_db(db_url: &str,name_book: &str,author_book:&str){
    let query="INSERT INTO books_db (name_book,author_book) VALUES($1,$2)";

    let instances = SqlitePool::connect(&db_url).await.unwrap();

    let execute_query=sqlx::query(&query).bind(name_book).bind(author_book).execute(&instances).await;


    instances.close().await;
    print!("{:?}",execute_query);
}


pub  async fn select_data_db(db_url:&str,id_books:&u32){

    let query="SELECT * FROM books_db  WHERE id_book=$1";

    let instances = SqlitePool::connect(&db_url).await.unwrap();

    let buffer_exec=sqlx::query(&query).bind(id_books).fetch_all(&instances).await.unwrap();
	let str_result = buffer_exec
		.iter() //unpack the Vec<>
		.map(|r| format!("{} - {}", r.get::<i32, _>("id_book"), r.get::<String, _>("name_book"))) //mapping the SqliteRow
		.collect::<Vec<String>>()
		.join(", ");

	println!("\n== select tickets with Row:\n{}", str_result);


    instances.close().await;



}