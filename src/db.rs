use dotenv::dotenv;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};
use std::env;

pub async fn connect_and_test() -> mongodb::error::Result<()> {
    //find the .env file by dotenv
    dotenv().ok();

    //then access it using env::var(the_name_used_in_env)
    //the expect()used for error if it didn't find it
    let url = env::var("MONGODB_URL").expect("MONGODB_URL not found");
    //Client::with_uri_str --> this create the mongodb clint form the connection string that is in the .env
    let client = Client::with_uri_str(&url).await?;

    // Insert
    let docs = vec![
        doc! {"title": "osman "},
        doc! {"title": "ghani "},
        doc! {"title": "granday "},
    ];
    let coll: Collection<Document> = client.database("bitWallet").collection("names");
    let data = coll.insert_many(docs).await?;
    println!("the data: {:?}", data);
    let filter = doc! {"title": "osman "};
    let mut cursor = coll.find(filter).await?;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => println!("Found: {:?}", doc),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    let the_delete = doc! {};
    coll.delete_many(the_delete).await?;
    println!("connected to the database!");
    Ok(())
}
