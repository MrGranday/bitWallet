use anyhow::{Context, Result};
use bdk::bitcoin::network::constants::Network;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::Address;
use bdk::blockchain::EsploraBlockchain;
use bdk::database::MemoryDatabase;
use bdk::wallet::AddressIndex;
use bdk::{SignOptions, SyncOptions, Wallet};
use std::env;
use std::str::FromStr;

pub struct WalletHandle {
    pub wallet: Wallet<MemoryDatabase>,
    pub network: Network,
    pub esplora_client: EsploraBlockchain,
}

pub fn init_hot_wallet() -> Result<WalletHandle> {
    dotenvy::dotenv().ok();

    let network_str = env::var("NETWORK").unwrap_or_else(|_| "Testnet".to_string());
    let network = match network_str.as_str() {
        "Testnet" => Network::Testnet,
        "Mainnet" => Network::Bitcoin,
        "Signet" => Network::Signet,
        "Regtest" => Network::Regtest,
        _ => Network::Testnet,
    };

    let esplora_url = env::var("ESPLORA_URL").unwrap_or_else(|_| "https://mutinynet.com/api".to_string());
    let esplora_client = EsploraBlockchain::new(&esplora_url, 20);

    // We use a fixed descriptor from .env for the hot wallet.
    // If none exists, we fallback to a hardcoded testnet descriptor for testing purposes.
    // WARNING: DO NOT USE THIS HARDCODED KEY WITH REAL FUNDS.
    let descriptor = env::var("HOT_WALLET_DESCRIPTOR").unwrap_or_else(|_| {
        "wpkh(tprv8ZgxMBicQKsPcx5nBGsR63Pe8DoJCjqxVDnS4d8B7C72eYFDRS4sM9u2n9qZ4QCRK7g2t2qJ2aL8A6r9wWpZ4vYpEEDpP5P5r1Rj5P5q1P/84'/1'/0'/0/*)".to_string()
    });

    let wallet = Wallet::new(
        &descriptor,
        Some(&descriptor.replace("/0/*", "/1/*")), // Change descriptor
        network,
        MemoryDatabase::default(),
    )
    .context("Failed to initialize wallet")?;

    Ok(WalletHandle {
        wallet,
        network,
        esplora_client,
    })
}

pub fn generate_deposit_address(handle: &WalletHandle) -> Result<String> {
    let address = handle.wallet.get_address(AddressIndex::New)?;
    Ok(address.address.to_string())
}

pub async fn sync_wallet(handle: &WalletHandle) -> Result<()> {
    handle
        .wallet
        .sync(&handle.esplora_client, SyncOptions::default())
        .await
        .context("Failed to sync wallet with Esplora backend")?;
    Ok(())
}

pub fn get_wallet_balance(handle: &WalletHandle) -> Result<u64> {
    let balance = handle.wallet.get_balance()?;
    Ok(balance.get_spendable())
}

pub async fn withdraw_on_chain(
    handle: &WalletHandle,
    to_address: &str,
    amount_sats: u64,
) -> Result<String> {
    let to_addr = Address::from_str(to_address)
        .context("Invalid recipient address")?;

    let mut tx_builder = handle.wallet.build_tx();
    tx_builder.add_recipient(to_addr.script_pubkey(), amount_sats);
    
    // We add a fee rate. bdk allows setting it directly or estimating. We will set a conservative fixed fee rate of 5 sat/vB for testnet.
    tx_builder.fee_rate(bdk::FeeRate::from_sat_per_vb(5.0));

    let (mut psbt, _details) = tx_builder
        .finish()
        .context("Failed to build transaction (insufficient funds?)")?;

    let finalized = handle
        .wallet
        .sign(&mut psbt, SignOptions::default())
        .context("Failed to sign transaction")?;

    if !finalized {
        anyhow::bail!("Transaction was not completely signed");
    }

    let raw_tx = psbt.extract_tx();
    let txid = raw_tx.txid();

    handle
        .esplora_client
        .broadcast(&raw_tx)
        .await
        .context("Failed to broadcast transaction")?;

    Ok(txid.to_string())
}
