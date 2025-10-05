mod db;
use dotenv::dotenv;
use mongodb::{Client, Collection};
use std::env;

use db::{
    delete_user, find_user, find_user_by_email, insert_user, update_user_balance, Transaction,
    WalletUser,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // access the .env
    dotenv().ok();

    //then access this specific variable (MONGODB_URL) and then if it didnt find it
    //expect() to show the error message
    let url = env::var("MONGODB_URL").expect("couldn't find it ");
    let clint = Client::with_uri_str(&url).await?;
    let my_coll: Collection<WalletUser> = clint.database("bitWallet").collection("users");
    let tx_coll: Collection<Transaction> = clint.database("bitWallet").collection("transaction");

    // insert the data in here
    let user1 = WalletUser {
        name: "Osman".to_string(),
        balance: 100.0,
        email: "osman@gmail.com".to_string(),
        password: "12345".to_string(),
    };
    println!("connected to database: ");
    //insert user
    insert_user(&my_coll, user1).await?;
    println!("find the user");
    find_user(&my_coll).await?;
    println!("deleted the user");
    delete_user(&my_coll).await?;
    find_user(&my_coll).await?;

    Ok(())
}
