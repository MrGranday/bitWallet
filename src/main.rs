mod db;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    db::connect_and_test().await?;
    println!("connected to the database!");
    Ok(())
}
