use dotenv::dotenv;
use std::env;

pub struct WalletHandle {
    // the actual wallet object from bdk
    //stores UTXOs,keys,transaction cache, and etc
    pub wallet: bdk::Wallet<bdk::database::MemoryDatabase>,
    //Identifier string for this wallet
    pub wallet_name: String,
    //Enum representing the network (Testnet/Mainnet/Regtest).
    pub network: bdk::bitcoin::Network,
    //Descriptor formula for receiving addresses.
    pub descriptor_receive: String,
    //Descriptor formula for change addresses.
    pub descriptor_change: String,
    //Esplora endpoint for blockchain data.
    pub backend_url: String,
    //When the wallet was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
}

//ChainConfig struct defines where and how wallet connects to the blockchain network.
pub struct ChainConfig {
    pub network: bdk::bitcoin::Network,
    pub esplora_url: String,
    pub retry_attempts: u32,
    pub timeout_secs: u64,
    pub use_proxy: bool,
}

pub async fn create_or_restore_wallet() {
    dotenv().ok();
    let esplora_url_load = env::var("ESPLORA_URL").expect("can't find the esplora api variable");
    let network = env::var("NETWORK").expect("can't find the network variable");
    let retry_attempts =
        env::var("RETRY_ATTEMPTS").expect("can't find the retry attempts variable");
    let timeout_secs = env::var("TIMEOUT_SECS").expect("can't find the the timeout sec variable");
    let use_proxy = env::var("USE_PROXY").expect("can't find the use proxy variable");
}
