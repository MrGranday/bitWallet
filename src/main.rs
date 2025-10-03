use dotenv::dotenv;
use std::env;

async fn the_main() {
    println!("hello osman ");
}
#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo = env::var("MONGODB_URL");

    match mongo {
        Ok(val) => println!("MONGODB_URL: {:?}", val),
        Err(e) => println!("Error MONGODB_URL: {:?}", e),
    }
    the_main().await;
    println!("Hello, world!");
}
