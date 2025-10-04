use bdk::electrum_client::client;
use dotenv::dotenv;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    error::Result,
    Client, Collection,
};
use std::{collections, env};

// async fn the_main() {
//     println!("hello osman ");
// }

// async fn mongo_connect() {}
#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();

    //env::var() this returns the result and this means success or failure
    let url = env::var("MONGODB_URL").expect("msg");

    let client = Client::with_uri_str(&url).await?;
    // the_main().await;

    let docs = vec![
        doc! {"title": "osman "},
        doc! {"title": "ghani "},
        doc! {"title": "granday "},
    ];
    let my_coll: Collection<Document> = client.database("bitWallet").collection("names");
    let data = my_coll.insert_many(docs).await?;
    println!("the data: {:?}", data);
    let filter = doc! {"title": "osman "};
    let mut cursor = my_coll.find(filter).await?;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => println!("Found: {:?}", doc),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    let the_delete = doc! {};
    my_coll.delete_many(the_delete).await?;
    println!("connected to the database!");
    Ok(())
}
