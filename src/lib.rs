pub mod asset;
pub mod ledger;
pub mod payment;
pub mod identity;

use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Signature};

/// Core types and structures used throughout the SDK
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub name: String,
    pub description: String,
    pub value: u64,
    pub issuer: Pubkey,
    pub asset_type: String,
    pub created_at: u64,
    pub token_mint: Option<Pubkey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub signature: Signature,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub token_mint: Pubkey,
    pub timestamp: u64,
    pub memo: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FinternetConfig {
    pub rpc_url: String,
    pub commitment_level: String,
}

impl Default for FinternetConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://api.devnet.solana.com".to_string(),
            commitment_level: "confirmed".to_string(),
        }
    }
}

/// Main SDK client
pub struct FinternetClient {
    pub config: FinternetConfig,
    pub client: solana_client::rpc_client::RpcClient,
}

impl FinternetClient {
    pub fn new(config: FinternetConfig) -> Self {
        let client = solana_client::rpc_client::RpcClient::new(&config.rpc_url);
        Self { config, client }
    }
    
    pub fn new_devnet() -> Self {
        Self::new(FinternetConfig::default())
    }
}

// Re-export main functionality
pub use payment::*;
pub use identity::*; 