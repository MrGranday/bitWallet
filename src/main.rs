mod db;
use dotenv::dotenv;
use mongodb::{Client, Collection};
use std::env;

use db::{
    check_balance, delete_user, find_user_by_email, get_user_transaction, insert_user, login_user,
    transfer_fund, update_user_balance, Transaction, TransactionLogs, WalletUser,
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
    let tx_logs_coll: Collection<TransactionLogs> =
        clint.database("bitWallet").collection("transaction_logs");

    // insert the data in here
    let user1 = WalletUser {
        name: "Osman".to_string(),
        balance: 100.0,
        email: "osman@gmail.com".to_string(),
        password: "12345".to_string(),
    };
    let user2 = WalletUser {
        name: "ali".to_string(),
        balance: 100.0,
        email: "ali@gmail.com".to_string(),
        password: "12345".to_string(),
    };
    println!("connected to database: ");
    //insert user
    insert_user(&my_coll, user1).await?;
    insert_user(&my_coll, user2).await?;
    transfer_fund(&my_coll, &tx_coll, "osman@gmail.com", "ali@gmail.com", 20.0).await?;
    find_user_by_email(&my_coll, "osman@gmail.com").await?;
    find_user_by_email(&my_coll, "ali@gmail.com").await?;
    delete_user(&my_coll, "osman@gmail.com").await?;
    delete_user(&my_coll, "ali@gmail.com").await?;
    find_user_by_email(&my_coll, "osman@gmail.com").await?;
    find_user_by_email(&my_coll, "ali@gmail.com").await?;
    check_balance(&my_coll, "ali@gmail.com").await?;
    check_balance(&my_coll, "osman@gmail.com").await?;
    get_user_transaction(&tx_coll, "ali@gmail.com").await?;
    get_user_transaction(&tx_coll, "osman@gmail.com").await?;

    login_user(&my_coll, "osman@gmail.com", "12345").await?;

    // find_user(&my_coll).await?;

    Ok(())
}
