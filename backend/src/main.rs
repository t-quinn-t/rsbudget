use backend::errors::Error;

use dotenv;

use sqlx::Pool;
use sqlx::postgres::Postgres;
use sqlx::query;

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    // Config database
    let db_url = std::env::var("DATABASE_URL")?;
    dbg!(db_url);
    let mut db_pool: Pool<Postgres> = Pool::connect(&db_url).await?;
    let rows = query!("SELECT 1 AS one").fetch_one(&mut db_pool).await?;

    // Setup server
    let mut app = tide::new();
    app.at("/").get(|_| async move {Ok("Hello World")});
    app.listen("127.0.0.1:2048").await?;

    Ok(())
}


