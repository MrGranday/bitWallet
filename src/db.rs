use dotenv::dotenv_iter;
use futures::{stream::Filter, StreamExt};
use mongodb::{bson::doc, bson::Document, Collection};
use serde::{Deserialize, Serialize};

//ok this will be for the user
#[derive(Debug, Serialize, Deserialize)]
//user collection
pub struct WalletUser {
    pub name: String,
    pub balance: f64,
    pub email: String,
    pub password: String,
}

// transaction collection
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: String,
}

//if i want to insert a new user in the collection
///insert new user
pub async fn insert_user(
    coll: &Collection<WalletUser>,
    user: WalletUser,
) -> mongodb::error::Result<()> {
    coll.insert_one(user).await?;
    Ok(())
}
// find all users and print it
///find all users
pub async fn find_user(coll: &Collection<WalletUser>) -> mongodb::error::Result<()> {
    let filter = doc! {"name":"Osman"};
    let mut curser = coll.find(filter).await?;
    while let Some(result) = curser.next().await {
        match result {
            Ok(user) => println!("Found the user: {:?}", user),
            Err(e) => println!("errr: {:?}", e),
            _ => println!("null"),
        }
    }
    Ok(())
}

pub async fn delete_user(coll: &Collection<WalletUser>) -> mongodb::error::Result<()> {
    coll.delete_many(doc! {}).await?;
    Ok(())
}

pub async fn update_user_balance(
    coll: &Collection<WalletUser>,
    name: &str,
    new_balance: f64,
) -> mongodb::error::Result<()> {
    coll.update_one(doc! {"name": name}, doc! {"$set":{"balance": new_balance}})
        .await?;
    Ok(())
}

pub async fn find_user_by_email(coll: &Collection<WalletUser>) -> mongodb::error::Result<()> {
    let filter = doc! {"email":"osman@gmail.com"};
    let mut curser = coll.find(filter).await?;
    while let Some(result) = curser.next().await {
        match result {
            Ok(user) => println!("user by email found: {:?}", user),
            Err(e) => println!("the error {:?}", e),
        }
    }

    Ok(())
}

pub async fn create_transaction(
    coll: &Collection<Transaction>,
    from: &str,
    to: &str,
    amount: f64,
) -> mongodb::error::Result<()> {
    let tx = Transaction {
        from: from.to_string(),
        to: to.to_string(),
        amount,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    coll.insert_one(tx).await?;
    Ok(())
}

pub async fn transfer_fund(
    coll: &Collection<WalletUser>,
    sender_email: &str,
    receiver_email: &str,
    amount: f64,
) -> mongodb::error::Result<()> {
    let filter_for_sender = doc! {"email": sender_email};
    let filter_for_receiver = doc! {"email": receiver_email};
    let mut curser_sender = coll.find(filter_for_sender).await?;
    while let Some(result) = curser_sender.next().await {
        match result {
            Ok(user) => {
                if user.balance < amount {
                    println!("insufflation balance: {:?}", user.balance)
                } else {
                    let new_balance = user.balance - amount;
                    update_user_balance(coll, sender_email, new_balance).await;
                }
            }
            Err(e) => println!("the error {:?}", e),
        }
    }
    let mut curser_receiver = coll.find(filter_for_receiver).await?;
    while let Some(result) = curser_receiver.next().await {
        match result {
            Ok(user) => println!("fund received:{:?} ", user),
            Err(e) => println!("error receiving fund: {:?}", e),
        }
        let new_balance = user.balace + amount;
    }

    Ok(())
}
