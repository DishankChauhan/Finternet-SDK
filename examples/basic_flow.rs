use anyhow::Result;
use finternet_sdk::{FinternetClient};
use log::info;
use solana_sdk::signer::Signer;
use std::collections::HashMap;

/// Comprehensive example demonstrating the Finternet SDK's core functionality
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("🚀 Starting Finternet SDK Basic Flow Demo");
    
    // Initialize client
    let client = FinternetClient::new_devnet();
    println!("📡 Connected to Solana devnet: {}", client.config.rpc_url);
    
    // Load or create wallet
    let wallet = match FinternetClient::load_default_wallet() {
        Ok(wallet) => {
            println!("🔑 Loaded existing wallet: {}", wallet.pubkey());
            wallet
        }
        Err(_) => {
            println!("🔑 Creating new demo wallet...");
            let new_wallet = FinternetClient::create_new_wallet();
            println!("   ✅ Created!");
            println!("   Public Key: {}", new_wallet.pubkey());
            println!("   💡 In production, save this wallet securely!");
            new_wallet
        }
    };
    
    // Check SOL balance for gas fees
    println!("\n💰 Checking wallet balance...");
    let sol_balance = client.client.get_balance(&wallet.pubkey())?;
    println!("   SOL Balance: {:.4}", sol_balance as f64 / 1_000_000_000.0);
    
    if sol_balance < 10_000_000 { // Less than 0.01 SOL
        println!("   💡 Run: solana airdrop 2 {}", wallet.pubkey());
        println!("   💡 Waiting for airdrop before continuing...");
        // In a real demo, you might wait or exit here
    }
    
    // Demo 1: Asset Tokenization
    println!("\n🏠 DEMO 1: Asset Tokenization");
    println!("   📝 Tokenizing a real estate property...");
    
    match client.tokenize_asset(
        "Luxury Apartment in NYC",
        "A 2-bedroom luxury apartment in Manhattan, New York",
        2_500_000, // $2.5M value
        "real_estate",
        &wallet,
    ).await {
        Ok(result) => {
            println!("   ✅ Asset tokenized successfully!");
            println!("   🪙 Token Mint: {}", result.0);
            println!("   📋 Asset Name: {}", result.1.name);
            println!("   💰 Asset Value: ${}", result.1.value);
            println!("   📅 Created: {}", result.1.created_at);
        }
        Err(e) => {
            println!("   ❌ Failed to tokenize asset: {}", e);
            println!("   💡 This might fail on devnet due to program deployment requirements");
        }
    }
    
    // Demo 2: Identity Registration
    println!("\n🆔 DEMO 2: Identity Registration");
    println!("   📝 Registering on-chain identity...");
    
    let mut metadata = HashMap::new();
    metadata.insert("email".to_string(), "demo@finternet.com".to_string());
    metadata.insert("organization".to_string(), "Finternet Corp".to_string());
    metadata.insert("demo_user".to_string(), "true".to_string());
    
    match client.register_identity(
        &wallet,
        "Demo User",
        metadata,
    ).await {
        Ok(signature) => {
            println!("   ✅ Identity registered successfully!");
            println!("   📝 Transaction: {}", signature);
        }
        Err(e) => {
            println!("   ❌ Failed to register identity: {}", e);
        }
    }
    
    // Demo 3: Asset Discovery
    println!("\n🔍 DEMO 3: Asset Discovery");
    println!("   📝 Discovering owned assets...");
    
    match client.get_owned_assets(&wallet.pubkey()).await {
        Ok(assets) => {
            if assets.is_empty() {
                println!("   📭 No assets found");
            } else {
                println!("   📦 Found {} assets:", assets.len());
                for (i, (mint, balance)) in assets.iter().enumerate() {
                    println!("      {}. Mint: {} (Balance: {})", i + 1, mint, balance);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Failed to get assets: {}", e);
        }
    }
    
    // Demo 4: Cross-Border Payment Simulation
    println!("\n💸 DEMO 4: Cross-Border Payment Simulation");
    println!("   📝 Setting up payment demo...");
    
    // Create a recipient wallet for demo
    let recipient_wallet = FinternetClient::create_new_wallet();
    println!("   ✅ Created recipient wallet for demo");
    println!("   🔑 Recipient: {}", recipient_wallet.pubkey());
    
    // Check current USDC balance
    let usdc_balance = client.get_usdc_balance(&wallet.pubkey()).await?;
    println!("   💰 Current USDC Balance: ${:.2}", usdc_balance);
    
    if usdc_balance >= 1.0 {
        println!("   📤 Sending $1.00 USDC to recipient...");
        
        match client.send_usdc_payment(
            &wallet,
            &recipient_wallet.pubkey(),
            1.0,
            Some("Demo cross-border payment via Finternet SDK"),
        ).await {
            Ok(signature) => {
                println!("   ✅ Payment sent successfully!");
                println!("   📝 Transaction: {}", signature);
                println!("   💡 Check recipient balance to confirm");
            }
            Err(e) => {
                println!("   ❌ Payment failed: {}", e);
                println!("   💡 Make sure you have USDC in your wallet");
            }
        }
    } else {
        println!("   💡 Insufficient USDC for payment demo");
        println!("   💡 You would need USDC tokens for payments");
    }
    
    // Demo 5: Transaction History
    println!("\n📜 DEMO 5: Transaction History");
    println!("   📝 Fetching recent transactions...");
    
    match client.get_transaction_history(&wallet.pubkey(), Some(5)).await {
        Ok(history) => {
            if history.is_empty() {
                println!("   📭 No transactions found");
            } else {
                println!("   📋 Found {} recent transactions:", history.len());
                for (i, record) in history.iter().enumerate() {
                    println!("      {}. {} -> {} ({} tokens)", 
                        i + 1, 
                        record.from, 
                        record.to, 
                        record.amount
                    );
                    if let Some(memo) = &record.memo {
                        println!("         Memo: {}", memo);
                    }
                }
            }
        }
        Err(e) => {
            println!("   ❌ Failed to get transaction history: {}", e);
        }
    }
    
    // Demo 6: Wallet Information
    println!("\n🔍 DEMO 6: Wallet Information");
    println!("   📝 Getting detailed wallet info...");
    
    match client.get_wallet_info(&wallet.pubkey()).await {
        Ok(wallet_info) => {
            println!("   📊 Wallet Statistics:");
            println!("      SOL Balance: {:.4}", wallet_info.sol_balance_as_sol());
            println!("      Token Types: {}", wallet_info.total_token_types());
            
            // Try to get identity info
            if let Ok(identity) = client.get_identity(&wallet.pubkey()).await {
                if let Some(display_name) = &identity.display_name {
                    println!("      Identity: {}", display_name);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Failed to get wallet info: {}", e);
        }
    }
    
    println!("\n🎉 Demo completed!");
    println!("   📚 This demonstrated:");
    println!("   • Asset tokenization on Solana");
    println!("   • On-chain identity registration");
    println!("   • Asset discovery and management");
    println!("   • Cross-border payments with USDC");
    println!("   • Transaction history tracking");
    println!("   • Unified wallet information");
    println!("\n💡 For production use:");
    println!("   • Use mainnet configuration");
    println!("   • Implement proper error handling");
    println!("   • Add transaction confirmation waits");
    println!("   • Use secure wallet storage");
    
    Ok(())
} 