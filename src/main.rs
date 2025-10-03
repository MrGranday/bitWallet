async fn the_main() {
    println!("hello osman ");
}
#[tokio::main]
async fn main() {
    the_main().await;
    println!("Hello, world!");
}
