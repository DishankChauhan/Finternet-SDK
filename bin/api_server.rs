use axum::{
    extract::{Json, Path, Query},
    http::{StatusCode, Method},
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use finternet_sdk::{FinternetClient, AssetMetadata, TransactionRecord};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
struct TokenizeAssetRequest {
    name: String,
    description: String,
    value: u64,
    asset_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenizeAssetResponse {
    mint: String,
    signature: String,
    metadata: AssetMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct SendPaymentRequest {
    to: String,
    amount: f64,
    memo: Option<String>,
    token_mint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SendPaymentResponse {
    signature: String,
    from: String,
    to: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct WalletInfoResponse {
    public_key: String,
    sol_balance: f64,
    usdc_balance: f64,
    token_accounts: Vec<TokenAccountInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenAccountInfo {
    mint: String,
    balance: u64,
    decimals: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetInfo {
    mint: String,
    metadata: Option<AssetMetadata>,
    balance: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

// Global client instance (in production, you'd want proper state management)
static mut CLIENT: Option<FinternetClient> = None;
static mut WALLET: Option<Keypair> = None;

async fn initialize_client() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        CLIENT = Some(FinternetClient::new_devnet());
        WALLET = Some(FinternetClient::load_default_wallet()?);
    }
    Ok(())
}

fn get_client() -> &'static FinternetClient {
    unsafe { CLIENT.as_ref().expect("Client not initialized") }
}

fn get_wallet() -> &'static Keypair {
    unsafe { WALLET.as_ref().expect("Wallet not initialized") }
}

async fn health_check() -> ResponseJson<serde_json::Value> {
    ResponseJson(serde_json::json!({
        "status": "healthy",
        "service": "Finternet SDK API",
        "version": "0.1.0"
    }))
}

async fn tokenize_asset(
    Json(payload): Json<TokenizeAssetRequest>,
) -> Result<ResponseJson<TokenizeAssetResponse>, StatusCode> {
    let client = get_client();
    let wallet = get_wallet();

    match client
        .tokenize_asset(
            &payload.name,
            &payload.description,
            payload.value,
            &payload.asset_type,
            wallet,
        )
        .await
    {
        Ok((mint, metadata, signature)) => {
            println!("✅ Token created: {} with signature: {}", mint, signature);
            
            // Wait for blockchain confirmation (20 seconds)
            println!("⏳ Waiting for blockchain confirmation...");
            tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
            println!("✅ Blockchain confirmation wait completed");
            
            Ok(ResponseJson(TokenizeAssetResponse {
                mint: mint.to_string(),
                signature: signature.to_string(),
                metadata,
            }))
        }
        Err(e) => {
            eprintln!("Token creation failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn send_payment(
    Json(payload): Json<SendPaymentRequest>,
) -> Result<ResponseJson<SendPaymentResponse>, StatusCode> {
    let client = get_client();
    let wallet = get_wallet();

    let to_pubkey = match Pubkey::from_str(&payload.to) {
        Ok(pk) => pk,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let result = if let Some(token_mint) = payload.token_mint {
        let mint_pubkey = match Pubkey::from_str(&token_mint) {
            Ok(pk) => pk,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        };
        // Convert f64 to u64 for SPL token amount (assuming 9 decimals for most SPL tokens)
        let amount_lamports = (payload.amount * 1_000_000_000.0) as u64;
        client
            .send_payment(
                wallet,
                &to_pubkey,
                amount_lamports,
                &mint_pubkey,
                payload.memo.as_deref(),
            )
            .await
    } else {
        // For USDC payments, the SDK handles the conversion internally
        client
            .send_usdc_payment(wallet, &to_pubkey, payload.amount, payload.memo.as_deref())
            .await
    };

    match result {
        Ok(signature) => Ok(ResponseJson(SendPaymentResponse {
            signature: signature.to_string(),
            from: wallet.pubkey().to_string(),
            to: payload.to,
            amount: payload.amount,
        })),
        Err(e) => {
            eprintln!("Payment failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_wallet_info() -> Result<ResponseJson<WalletInfoResponse>, StatusCode> {
    let client = get_client();
    let wallet = get_wallet();

    let sol_balance = match client.get_sol_balance(&wallet.pubkey()).await {
        Ok(balance) => balance,
        Err(_) => 0.0,
    };

    let usdc_balance = match client.get_usdc_balance(&wallet.pubkey()).await {
        Ok(balance) => balance,
        Err(_) => 0.0,
    };

    let token_accounts = match client.get_token_accounts(&wallet.pubkey()).await {
        Ok(accounts) => accounts
            .into_iter()
            .map(|(mint, balance)| TokenAccountInfo {
                mint: mint.to_string(),
                balance,
                decimals: 9, // Default for most SPL tokens
            })
            .collect(),
        Err(_) => vec![],
    };

    Ok(ResponseJson(WalletInfoResponse {
        public_key: wallet.pubkey().to_string(),
        sol_balance,
        usdc_balance,
        token_accounts,
    }))
}

async fn get_owned_assets() -> Result<ResponseJson<Vec<AssetInfo>>, StatusCode> {
    let client = get_client();
    let wallet = get_wallet();

    match client.get_owned_assets(&wallet.pubkey()).await {
        Ok(assets) => {
            let mut asset_infos = Vec::new();
            for (mint, balance) in assets {
                let metadata = client.get_asset_info(&mint).await.ok();
                asset_infos.push(AssetInfo {
                    mint: mint.to_string(),
                    metadata,
                    balance,
                });
            }
            Ok(ResponseJson(asset_infos))
        }
        Err(e) => {
            eprintln!("Failed to get assets: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_transaction_history() -> Result<ResponseJson<Vec<TransactionRecord>>, StatusCode> {
    let client = get_client();
    let wallet = get_wallet();

    match client
        .get_transaction_history(&wallet.pubkey(), Some(20))
        .await
    {
        Ok(history) => Ok(ResponseJson(history)),
        Err(e) => {
            eprintln!("Failed to get transaction history: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_asset_info(Path(mint_address): Path<String>) -> Result<ResponseJson<AssetMetadata>, StatusCode> {
    let client = get_client();
    
    let mint_pubkey = match Pubkey::from_str(&mint_address) {
        Ok(pk) => pk,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    match client.get_asset_info(&mint_pubkey).await {
        Ok(metadata) => Ok(ResponseJson(metadata)),
        Err(e) => {
            eprintln!("Failed to get asset info: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize the Finternet client and wallet
    initialize_client().await?;

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    // Build the router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/tokenize-asset", post(tokenize_asset))
        .route("/api/send-payment", post(send_payment))
        .route("/api/wallet-info", get(get_wallet_info))
        .route("/api/assets", get(get_owned_assets))
        .route("/api/transactions", get(get_transaction_history))
        .route("/api/asset/:mint_address", get(get_asset_info))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    
    println!("🚀 Finternet SDK API Server running on http://127.0.0.1:3001");
    println!("📊 Health check: http://127.0.0.1:3001/health");
    println!("🌐 Frontend should run on http://localhost:3000");
    println!("🔗 CORS enabled for frontend integration");

    axum::serve(listener, app).await?;

    Ok(())
} 