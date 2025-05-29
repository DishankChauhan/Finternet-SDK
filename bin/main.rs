use anyhow::Result;
use clap::{Parser, Subcommand};
use finternet_sdk::{FinternetClient, FinternetConfig};
use log::info;
use solana_sdk::{pubkey::Pubkey, signer::Signer};
use std::collections::HashMap;
use std::str::FromStr;

// USDC constants for CLI usage
const USDC_DEVNET_MINT: &str = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";

fn usdc_devnet_mint() -> solana_sdk::pubkey::Pubkey {
    USDC_DEVNET_MINT.parse().unwrap()
}

#[derive(Parser)]
#[command(name = "finternet-cli")]
#[command(about = "A CLI for the Finternet SDK - Tokenize assets, send payments, and interact with the unified ledger")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(long, help = "RPC URL for Solana (defaults to devnet)")]
    rpc_url: Option<String>,
    
    #[arg(long, help = "Path to wallet file (defaults to ~/.config/solana/id.json)")]
    wallet: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Tokenize a real-world or digital asset
    TokenizeAsset {
        #[arg(short, long)]
        name: String,
        
        #[arg(short, long)]
        description: String,
        
        #[arg(short, long)]
        value: u64,
        
        #[arg(short, long, default_value = "real_estate")]
        asset_type: String,
    },
    
    /// Send USDC payment to another wallet
    SendPayment {
        #[arg(short, long)]
        to: String, // Public key as string
        
        #[arg(short, long)]
        amount: f64, // Amount in USDC
        
        #[arg(short, long)]
        memo: Option<String>,
    },
    
    /// Send any SPL token payment
    SendToken {
        #[arg(short, long)]
        to: String, // Public key as string
        
        #[arg(short, long)]
        amount: u64, // Amount in token lamports
        
        #[arg(short, long)]
        token_mint: String, // Token mint address
        
        #[arg(short, long)]
        memo: Option<String>,
    },
    
    /// Get transaction history for wallet
    History {
        #[arg(short, long, default_value = "10")]
        limit: usize,
        
        #[arg(short, long)]
        address: Option<String>, // If not provided, uses wallet address
    },
    
    /// Get asset information by mint address
    AssetInfo {
        #[arg(short, long)]
        mint: String,
    },
    
    /// Get wallet balance information
    Balance {
        #[arg(short, long)]
        address: Option<String>, // If not provided, uses wallet address
    },
    
    /// Get owned assets for a wallet
    Assets {
        #[arg(short, long)]
        address: Option<String>, // If not provided, uses wallet address
    },
    
    /// Create a new wallet
    CreateWallet {
        #[arg(short, long)]
        output_path: String,
    },
    
    /// Get wallet info and identity
    WalletInfo {
        #[arg(short, long)]
        address: Option<String>,
    },
    
    /// Register identity on-chain
    RegisterIdentity {
        #[arg(short, long)]
        display_name: String,
        
        #[arg(short, long)]
        email: Option<String>,
        
        #[arg(short, long)]
        organization: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    // Initialize Finternet client
    let config = if let Some(rpc_url) = cli.rpc_url {
        FinternetConfig {
            rpc_url,
            commitment_level: "confirmed".to_string(),
        }
    } else {
        FinternetConfig::default()
    };
    
    let client = FinternetClient::new(config);
    info!("Connected to Solana RPC: {}", client.config.rpc_url);
    
    // Load wallet
    let wallet = if let Some(wallet_path) = cli.wallet {
        FinternetClient::load_wallet_from_file(&std::path::Path::new(&wallet_path))?
    } else {
        match FinternetClient::load_default_wallet() {
            Ok(wallet) => wallet,
            Err(_) => {
                println!("⚠️  No wallet found. Creating a new one...");
                let new_wallet = FinternetClient::create_new_wallet();
                println!("🔑 New wallet created: {}", new_wallet.pubkey());
                println!("💡 Save this wallet using: finternet-cli create-wallet -o ~/.config/solana/id.json");
                println!("💰 Don't forget to airdrop SOL for gas fees!");
                new_wallet
            }
        }
    };
    
    println!("🔑 Using wallet: {}", wallet.pubkey());
    
    // Execute commands
    match cli.command {
        Commands::TokenizeAsset { name, description, value, asset_type } => {
            println!("🏭 Tokenizing asset: {}", name);
            
            let (mint_address, metadata) = client
                .tokenize_asset(&name, &description, value, &asset_type, &wallet)
                .await?;
            
            println!("✅ Asset tokenized successfully!");
            println!("🪙 Mint Address: {}", mint_address);
            println!("📋 Metadata: {:#?}", metadata);
        }
        
        Commands::SendPayment { to, amount, memo } => {
            let to_pubkey = Pubkey::from_str(&to)?;
            println!("💸 Sending ${:.2} USDC to {}", amount, to);
            
            let signature = client
                .send_usdc_payment(&wallet, &to_pubkey, amount, memo.as_deref())
                .await?;
            
            println!("✅ Payment sent successfully!");
            println!("📝 Transaction: {}", signature);
        }
        
        Commands::SendToken { to, amount, token_mint, memo } => {
            let to_pubkey = Pubkey::from_str(&to)?;
            let mint_pubkey = Pubkey::from_str(&token_mint)?;
            
            println!("🪙 Sending {} tokens to {}", amount, to);
            
            let signature = client
                .send_payment(&wallet, &to_pubkey, amount, &mint_pubkey, memo.as_deref())
                .await?;
            
            println!("✅ Token transfer successful!");
            println!("📝 Transaction: {}", signature);
        }
        
        Commands::History { limit, address } => {
            let target_address = if let Some(addr) = address {
                Pubkey::from_str(&addr)?
            } else {
                wallet.pubkey()
            };
            
            println!("📜 Fetching transaction history for: {}", target_address);
            
            let history = client.get_transaction_history(&target_address, Some(limit)).await?;
            
            if history.is_empty() {
                println!("📭 No transactions found");
            } else {
                println!("📋 Found {} transactions:", history.len());
                for (i, record) in history.iter().enumerate() {
                    println!("\n{}. Transaction: {}", i + 1, record.signature);
                    println!("   From: {}", record.from);
                    println!("   To: {}", record.to);
                    println!("   Amount: {}", record.amount);
                    println!("   Token: {}", record.token_mint);
                    if let Some(memo) = &record.memo {
                        println!("   Memo: {}", memo);
                    }
                    println!("   Time: {}", record.timestamp);
                }
            }
        }
        
        Commands::AssetInfo { mint } => {
            let mint_pubkey = Pubkey::from_str(&mint)?;
            println!("🔍 Fetching asset info for: {}", mint);
            
            let asset_info = client.get_asset_info(&mint_pubkey).await?;
            println!("📋 Asset Information:");
            println!("   Name: {}", asset_info.name);
            println!("   Description: {}", asset_info.description);
            println!("   Value: {}", asset_info.value);
            println!("   Issuer: {}", asset_info.issuer);
            println!("   Type: {}", asset_info.asset_type);
            println!("   Created: {}", asset_info.created_at);
        }
        
        Commands::Balance { address } => {
            let target_address = if let Some(addr) = address {
                Pubkey::from_str(&addr)?
            } else {
                wallet.pubkey()
            };
            
            println!("💰 Checking balances for: {}", target_address);
            
            // Get SOL balance
            let sol_balance = client.client.get_balance(&target_address)?;
            println!("   SOL: {:.4}", sol_balance as f64 / 1_000_000_000.0);
            
            // Get USDC balance
            let usdc_balance = client.get_usdc_balance(&target_address).await?;
            println!("   USDC: ${:.2}", usdc_balance);
            
            // Get other token balances
            let token_accounts = client.get_token_accounts(&target_address).await?;
            if !token_accounts.is_empty() {
                println!("\n🪙 Other tokens:");
                for (mint, balance) in token_accounts {
                    if mint != usdc_devnet_mint() {
                        println!("   {}: {}", mint, balance);
                    }
                }
            }
        }
        
        Commands::Assets { address } => {
            let target_address = if let Some(addr) = address {
                Pubkey::from_str(&addr)?
            } else {
                wallet.pubkey()
            };
            
            println!("🏦 Fetching owned assets for: {}", target_address);
            
            let assets = client.get_owned_assets(&target_address).await?;
            
            if assets.is_empty() {
                println!("📭 No assets found");
            } else {
                println!("📋 Found {} assets:", assets.len());
                for (i, (mint, balance)) in assets.iter().enumerate() {
                    println!("\n{}. Mint: {}", i + 1, mint);
                    println!("   Balance: {}", balance);
                    
                    // Try to get asset metadata
                    if let Ok(asset_info) = client.get_asset_info(mint).await {
                        println!("   Name: {}", asset_info.name);
                        println!("   Type: {}", asset_info.asset_type);
                    }
                }
            }
        }
        
        Commands::CreateWallet { output_path } => {
            let new_wallet = FinternetClient::create_new_wallet();
            let path = std::path::Path::new(&output_path);
            
            FinternetClient::save_wallet_to_file(&new_wallet, path)?;
            
            println!("✅ New wallet created!");
            println!("🔑 Public Key: {}", new_wallet.pubkey());
            println!("📁 Saved to: {}", output_path);
            println!("\n💡 Next steps:");
            println!("   1. Airdrop SOL: solana airdrop 2 {}", new_wallet.pubkey());
            println!("   2. Set as default: solana config set --keypair {}", output_path);
        }
        
        Commands::WalletInfo { address } => {
            let target_address = if let Some(addr) = address {
                Pubkey::from_str(&addr)?
            } else {
                wallet.pubkey()
            };
            
            println!("ℹ️  Wallet Information for: {}", target_address);
            
            let wallet_info = client.get_wallet_info(&target_address).await?;
            let identity = client.get_identity(&target_address).await?;
            
            println!("\n💰 Balances:");
            println!("   SOL: {:.4}", wallet_info.sol_balance_as_sol());
            println!("   Token types: {}", wallet_info.total_token_types());
            
            println!("\n🆔 Identity:");
            if let Some(display_name) = &identity.display_name {
                println!("   Display Name: {}", display_name);
            }
            
            for (key, value) in &identity.metadata {
                println!("   {}: {}", key, value);
            }
            
            let is_active = client.is_account_active(&target_address).await?;
            println!("   Status: {}", if is_active { "Active" } else { "Inactive" });
        }
        
        Commands::RegisterIdentity { display_name, email, organization } => {
            let mut metadata = HashMap::new();
            
            if let Some(email) = email {
                metadata.insert("email".to_string(), email);
            }
            
            if let Some(org) = organization {
                metadata.insert("organization".to_string(), org);
            }
            
            metadata.insert("registration_method".to_string(), "cli".to_string());
            
            println!("📝 Registering identity: {}", display_name);
            
            let signature = client
                .register_identity(&wallet, &display_name, metadata)
                .await?;
            
            println!("✅ Identity registered successfully!");
            println!("📝 Transaction: {}", signature);
        }
    }
    
    Ok(())
} 