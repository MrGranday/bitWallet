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
