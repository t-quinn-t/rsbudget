use backend::errors::Error;

use dotenv;

use sqlx::{Pool};
use sqlx::sqlite::{Sqlite};
use sqlx::query;


#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    // Config database
    let db_url = std::env::var("DATABASE_URL")?;

    dbg!(&db_url);
    let db_pool: Pool<Sqlite> = Pool::connect(&db_url).await?;
    let rows = query!("select 1 as one").fetch_one(&db_pool).await?;
    dbg!(rows);

    // Setup server
    let mut app = tide::new();
    app.at("/").get(|_| async move {Ok("Hello World")});
    app.listen("127.0.0.1:2048").await?;

    Ok(())
}