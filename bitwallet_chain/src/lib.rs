pub struct WalletHandle {
    // the actual wallet object from bdk
    //stores UTXOs,keys,transaction cache, and etc
    pub wallet: bdk::Wallet<bdk::database::MemoryDatabase>,
    pub wallet_name: String,
    pub network: bdk::bitcoin::Network,
    pub descriptor_receive: String,
    pub descriptor_change: String,
    pub backend_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
