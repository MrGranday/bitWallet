use futures::StreamExt;
use mongodb::{bson::doc, bson::Document, Collection};
use serde::{Deserialize, Serialize};

//ok this will be for the user
#[derive(Debug, Serialize, Deserialize)]

pub struct WalletUser {
    pub name: String,
    pub balance: f64,
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
