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
#[derive(Debug, Serialize, Deserialize)]
// transaction logs
pub struct TransactionLogs {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub tx_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
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
// pub async fn find_user(coll: &Collection<WalletUser>) -> mongodb::error::Result<()> {
//     let filter = doc! {"name":"Osman"};
//     let mut curser = coll.find(filter).await?;
//     while let Some(result) = curser.next().await {
//         match result {
//             Ok(user) => println!("Found the user: {:?}", user),
//             Err(e) => println!("errr: {:?}", e),
//             _ => println!("null"),
//         }
//     }
//     Ok(())
// }

pub async fn delete_user(coll: &Collection<WalletUser>, email: &str) -> mongodb::error::Result<()> {
    coll.delete_many(doc! {"email": email}).await?;
    println!("user deleted: {:?}", email);

    Ok(())
}

pub async fn update_user_balance(
    coll: &Collection<WalletUser>,
    email: &str,
    new_balance: f64,
) -> mongodb::error::Result<()> {
    coll.update_one(
        doc! {"email": email},
        doc! {"$set": {"balance": new_balance}},
    )
    .await?;
    Ok(())
}

pub async fn find_user_by_email(
    coll: &Collection<WalletUser>,
    email: &str,
) -> mongodb::error::Result<()> {
    let filter = doc! {"email":email};
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
    tx_coll: &Collection<Transaction>,
    sender_email: &str,
    receiver_email: &str,
    amount: f64,
) -> mongodb::error::Result<()> {
    if let Some(user) = coll.find_one(doc! {"email": sender_email}).await? {
        if user.balance < amount {
            println!("Insufficient balance: {:?}", user.balance)
        } else {
            let new_balance = user.balance - amount;
            update_user_balance(coll, sender_email, new_balance).await?;
        }
    }
    if let Some(user) = coll.find_one(doc! {"email": receiver_email}).await? {
        let new_balance = user.balance + amount;
        update_user_balance(coll, receiver_email, new_balance).await?;
        println!("Fund received: {:?}", user.email);
        create_transaction(tx_coll, sender_email, receiver_email, amount).await?;
    } else {
        println!("Receiver not found");
    }

    Ok(())
}

pub async fn check_balance(
    coll: &Collection<WalletUser>,
    email: &str,
) -> mongodb::error::Result<()> {
    if let Some(user) = coll.find_one(doc! {"email": email}).await? {
        println!(
            "the user: {:?} , and the balance{:?}",
            user.email, user.balance
        )
    } else {
        println!("user not found !");
    }
    Ok(())
}

pub async fn get_user_transaction(
    coll: &Collection<Transaction>,
    email: &str,
) -> mongodb::error::Result<()> {
    let filter = doc! {"$or": [{"from": email}, {"to": email}]};
    let mut curser = coll.find(filter).await?;
    while let Some(result) = curser.next().await {
        match result {
            Ok(tx) => {
                if tx.from == email {
                    println!("you have sent: {:?}, to : {:?}", tx.amount, tx.to);
                } else {
                    println!("you have received :{:?}, from : {:?}", tx.amount, tx.from)
                }
            }
            Err(e) => println!("error reading the transaction{:?}", e),
        }
    }
    Ok(())
}

pub async fn login_user(
    coll: &Collection<WalletUser>,
    email: &str,
    password: &str,
) -> mongodb::error::Result<()> {
    if let Some(user) = coll
        .find_one(doc! {"$and":[ {"email":email},{"password":password}]})
        .await?
    {
        println!("Login successful: {:?}", user);
    } else {
        println!("Invalid email or password");
    }

    Ok(())
}

pub async fn deposit_fund(
    coll: &Collection<WalletUser>,
    email: &str,
    amount: f64,
) -> mongodb::error::Result<()> {
    if let Some(user) = coll.find_one(doc! {"email":email}).await? {
        let new_balance = user.balance + amount;
        println!("you deposit to :{:?}", user.email);
        println!("your current balance is :{:?}", user.balance);
        update_user_balance(coll, email, new_balance).await?;
    } else {
        println!("User not found ");
    }
    Ok(())
}

pub async fn withdraw_fund(
    coll: &Collection<WalletUser>,
    email: &str,
    amount: f64,
) -> mongodb::error::Result<()> {
    if let Some(user) = coll.find_one(doc! {"email":email}).await? {
        if amount <= user.balance {
            let new_balance = user.balance - amount;
            println!(
                "you withdraw: {:?} and your current balance is: {:?} ",
                amount, new_balance
            );
            update_user_balance(coll, email, new_balance).await?;
        } else {
            println!("insufficient balance your balance is : {:?}", user.balance)
        }
    }
    Ok(())
}

pub async fn log_transaction(
    coll: &Collection<TransactionLogs>,
    from: &str,
    to: &str,
    amount: f64,
    tx_type: &str,
) -> mongodb::error::Result<()> {
    let log = TransactionLogs {
        from: from.to_string(),
        to: to.to_string(),
        amount,
        tx_type: tx_type.to_string(),
        timestamp: chrono::Utc::now(),
    };

    coll.insert_one(log).await?;
    Ok(())
}
