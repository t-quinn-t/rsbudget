use dotenv;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("Hello, world!");
}
