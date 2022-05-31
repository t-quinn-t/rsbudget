use backend::errors::Error;

use dotenv;

use sqlx::{Pool, query};
use sqlx::sqlite::{Sqlite};
use tide::Request;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[derive(Clone)]
struct State {
    db_pool: Pool<Sqlite>
}

#[async_std::main]
async fn main() -> Result<(), Error> {

    // Setup logger and env var
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Config database
    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool: Pool<Sqlite> = Pool::connect(&db_url).await?;

    let state = State {db_pool};

    // Setup server and API endpoints
    let mut app = tide::with_state(state);
    app.at("/test").get(echo);
    app.listen("127.0.0.1:2048").await?;

    Ok(())
}

async fn echo(req: Request<State>) -> tide::http::Result<&'static str> {
    let db_pool = &req.state().db_pool;
    query!("select 1 as one").fetch_one(db_pool).await?;
    Ok("Test API request for the server.")
}