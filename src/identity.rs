use crate::FinternetClient;
use anyhow::{anyhow, Result};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FinternetIdentity {
    pub pubkey: Pubkey,
    pub display_name: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl FinternetIdentity {
    pub fn new(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            display_name: None,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_display_name(mut self, name: &str) -> Self {
        self.display_name = Some(name.to_string());
        self
    }
    
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

impl FinternetClient {
    /// Load wallet from the default Solana CLI location
    pub fn load_default_wallet() -> Result<Keypair> {
        let home_dir = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| anyhow!("Could not find home directory"))?;
        
        let wallet_path = Path::new(&home_dir)
            .join(".config")
            .join("solana")
            .join("id.json");
        
        Self::load_wallet_from_file(&wallet_path)
    }
    
    /// Load wallet from a specific file path
    pub fn load_wallet_from_file(path: &Path) -> Result<Keypair> {
        if !path.exists() {
            return Err(anyhow!("Wallet file does not exist: {}", path.display()));
        }
        
        let wallet_data = fs::read_to_string(path)?;
        let wallet_bytes: Vec<u8> = serde_json::from_str(&wallet_data)?;
        
        if wallet_bytes.len() != 64 {
            return Err(anyhow!("Invalid wallet file format"));
        }
        
        Ok(Keypair::from_bytes(&wallet_bytes)?)
    }
    
    /// Create a new random wallet
    pub fn create_new_wallet() -> Keypair {
        Keypair::new()
    }
    
    /// Save wallet to a file
    pub fn save_wallet_to_file(wallet: &Keypair, path: &Path) -> Result<()> {
        let wallet_bytes = wallet.to_bytes();
        let wallet_json = serde_json::to_string_pretty(&wallet_bytes.to_vec())?;
        
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(path, wallet_json)?;
        log::info!("Wallet saved to: {}", path.display());
        Ok(())
    }
    
    /// Get identity information for a public key
    pub async fn get_identity(&self, pubkey: &Pubkey) -> Result<FinternetIdentity> {
        log::info!("Getting identity for: {}", pubkey);
        
        // In a real implementation, this might query a DID registry or identity service
        // For now, we'll create a basic identity from the public key
        let mut identity = FinternetIdentity::new(*pubkey);
        
        // Try to get SOL balance as basic account verification
        match self.client.get_balance(pubkey) {
            Ok(balance) => {
                identity = identity.with_metadata("sol_balance", &balance.to_string());
                if balance > 0 {
                    identity = identity.with_metadata("account_status", "active");
                } else {
                    identity = identity.with_metadata("account_status", "inactive");
                }
            }
            Err(_) => {
                identity = identity.with_metadata("account_status", "not_found");
            }
        }
        
        // Check if this is a known system account
        if pubkey == &solana_sdk::system_program::id() {
            identity = identity.with_display_name("System Program");
        } else if pubkey == &spl_token::id() {
            identity = identity.with_display_name("SPL Token Program");
        }
        
        Ok(identity)
    }
    
    /// Register an identity with metadata (using memo transactions for simple on-chain storage)
    pub async fn register_identity(
        &self,
        wallet: &Keypair,
        display_name: &str,
        metadata: HashMap<String, String>,
    ) -> Result<Signature> {
        log::info!("Registering identity for: {}", wallet.pubkey());
        
        // Create identity registration data
        let identity_data = serde_json::json!({
            "action": "register_identity",
            "pubkey": wallet.pubkey().to_string(),
            "display_name": display_name,
            "metadata": metadata,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });
        
        // Write to ledger using memo
        self.write_ledger_entry(wallet, &identity_data.to_string()).await
    }
    
    /// Verify wallet ownership by signing a challenge
    pub fn verify_wallet_ownership(wallet: &Keypair, challenge: &str) -> Result<String> {
        let challenge_bytes = challenge.as_bytes();
        let signature = wallet.sign_message(challenge_bytes);
        Ok(signature.to_string())
    }
    
    /// Verify a signature against a public key and challenge
    pub fn verify_signature(pubkey: &Pubkey, challenge: &str, signature_str: &str) -> Result<bool> {
        let challenge_bytes = challenge.as_bytes();
        let signature: Signature = signature_str.parse()?;
        
        Ok(signature.verify(pubkey.as_ref(), challenge_bytes))
    }
    
    /// Get wallet information including balances
    pub async fn get_wallet_info(&self, pubkey: &Pubkey) -> Result<WalletInfo> {
        log::info!("Getting wallet info for: {}", pubkey);
        
        let sol_balance = self.client.get_balance(pubkey)?;
        let token_accounts = self.get_token_accounts(pubkey).await?;
        
        Ok(WalletInfo {
            pubkey: *pubkey,
            sol_balance,
            token_balances: token_accounts,
        })
    }
    
    /// Create a human-readable address from a public key
    pub fn create_readable_address(pubkey: &Pubkey, _prefix: &str) -> String {
        let pubkey_str = pubkey.to_string();
        format!("{}...{}", 
            &pubkey_str[..8], 
            &pubkey_str[pubkey_str.len()-8..]
        )
    }
    
    /// Check if an account is initialized and active
    pub async fn is_account_active(&self, pubkey: &Pubkey) -> Result<bool> {
        match self.client.get_account(pubkey) {
            Ok(account) => Ok(account.lamports > 0),
            Err(_) => Ok(false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WalletInfo {
    pub pubkey: Pubkey,
    pub sol_balance: u64,
    pub token_balances: HashMap<Pubkey, u64>,
}

impl WalletInfo {
    pub fn sol_balance_as_sol(&self) -> f64 {
        self.sol_balance as f64 / 1_000_000_000.0 // Convert lamports to SOL
    }
    
    pub fn has_tokens(&self) -> bool {
        !self.token_balances.is_empty()
    }
    
    pub fn total_token_types(&self) -> usize {
        self.token_balances.len()
    }
} 