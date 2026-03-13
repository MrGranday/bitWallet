mod db;
use dotenv::dotenv;
use mongodb::{Client, Collection};
use std::env;

use bitwallet_chain::{
    generate_deposit_address, get_wallet_balance, init_hot_wallet, sync_wallet, withdraw_on_chain,
};
use db::{
    assign_deposit_address, check_balance, credit_onchain_deposit, delete_user,
    find_user_by_deposit_address, find_user_by_email, get_user_transaction, insert_user,
    login_user, transfer_fund, update_user_balance, Transaction, TransactionLogs, WalletUser,
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
    // We will leave the connection and collection setup, and insert the demonstration of the Hot Wallet here.
    println!("connected to database.");

    // ==========================================================
    // 1. Initialize the BDK Hot Wallet
    // ==========================================================
    println!("Initializing Hot Wallet...");
    let wallet_handle = match init_hot_wallet() {
        Ok(w) => {
            println!("Hot wallet initialized successfully.");
            w
        }
        Err(e) => {
            println!("Failed to initialize hot wallet: {:?}", e);
            return Ok(());
        }
    };
    
    // Print the global balance of the hot wallet
    if let Ok(bal) = get_wallet_balance(&wallet_handle) {
        println!("Hot Wallet Balance: {} sats", bal);
    }

    // ==========================================================
    // 2. Generate a Deposit Address for a User
    // ==========================================================
    // Let's create an external user for demo purposes or use 'osman'.
    let user_email = "osman@gmail.com";
    
    // Check if Osman has a deposit address already, if not, generate one
    println!("Fetching deposit address for {}...", user_email);
    let mut address_to_watch = String::new();
    
    // In a real app we would check `find_user_by_email` first to see if `deposit_address` is `Some`.
    // For demo, we just generate a new one and overwrite.
    match generate_deposit_address(&wallet_handle) {
        Ok(address) => {
            println!("Generated new deposit address for {}: {}", user_email, address);
            assign_deposit_address(&my_coll, user_email, &address).await?;
            address_to_watch = address;
        }
        Err(e) => {
            println!("Failed to generate address: {:?}", e);
        }
    }

    // ==========================================================
    // 3. Sync Wallet and Check for Deposits
    // ==========================================================
    println!("Syncing Hot Wallet with Esplora (this may take a moment)...");
    if let Err(e) = sync_wallet(&wallet_handle).await {
        println!("Failed to sync wallet: {:?}", e);
    } else {
        println!("Wallet synced successfully.");
        if let Ok(bal) = get_wallet_balance(&wallet_handle) {
            println!("New Hot Wallet Balance: {} sats", bal);
        }
    }

    // In a full implementation, `sync_wallet` would return newly discovered UTXOs.
    // We would map those UTXO addresses to users via `find_user_by_deposit_address`
    // and then call `credit_onchain_deposit`.
    // Example:
    /*
    let discovered_amount_sats = 10000;
    let mock_txid = "0000000000000000000abcdef...";
    if let Ok(Some(user)) = find_user_by_deposit_address(&my_coll, &address_to_watch).await {
        println!("Detected deposit for user: {}", user.email);
        credit_onchain_deposit(&my_coll, &tx_logs_coll, &user.email, discovered_amount_sats, mock_txid).await?;
    }
    */

    // ==========================================================
    // 4. Withdraw On-Chain
    // ==========================================================
    // To test this, you must have actual Testnet sats in the hot wallet.
    /*
    println!("Requesting withdrawal...");
    let external_address = "tb1q..."; // put a real testnet address here
    let withdraw_amount_sats = 5000;
    match withdraw_on_chain(&wallet_handle, external_address, withdraw_amount_sats).await {
        Ok(txid) => println!("Withdrawal successful! TXID: {}", txid),
        Err(e) => println!("Withdrawal failed: {:?}", e),
    }
    */

    Ok(())
}
