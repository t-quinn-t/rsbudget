use rbudget_data::*;

use tokio::*;
extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    pretty_env_logger::init();
    let ds = DataStore::new().await?;
    Ok(())
}
