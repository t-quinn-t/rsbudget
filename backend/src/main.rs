use dotenv;
use backend::errors::Error;

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();

    // Setup server
    let mut app = tide::new();
    app.at("/").get(|_| async move {Ok("Hello World")});
    app.listen("127.0.0.1:2048").await?;

    Ok(())
}


