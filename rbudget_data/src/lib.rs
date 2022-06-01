use std::str::FromStr;

use errors::Error;

use dotenv::dotenv;
use sqlx::ConnectOptions;
use sqlx::Pool;
use sqlx::Sqlite;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;

pub mod errors;
mod repo;

pub struct DataStore {

}

impl DataStore {

    pub async fn new() -> Result<DataStore, Error> {

        dotenv().ok();  

        let db_url = dotenv::var("DATABASE_URL").unwrap();   
        dbg!(&db_url);  
        let db_pool: Pool<Sqlite> = Pool::connect_with(SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true)).await?;

        Ok(DataStore{})
    }
}

