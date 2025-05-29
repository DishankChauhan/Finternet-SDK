# 🌐 Finternet SDK - The Stripe for Tokenization + Payments 

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/solana-devnet-purple.svg)](https://solana.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **The complete Rust SDK for building tokenized finance applications on Finternet**  
> *Letting any developer plug into tokenized finance with just a few Rust calls.*

## 🎯 MVP Overview

This Rust-based Finternet SDK provides **real, end-to-end functionality** for:

- **🏦 Asset Tokenization** - Turn any real-world asset into SPL tokens on Solana
- **📋 Unified Ledger Access** - Read/write to a composable ledger of tokenized assets  
- **💸 Cross-Border Payments** - Send/receive USDC and SPL tokens with full auditability
- **🔍 Transaction Tracking** - Complete transaction history and settlement tracking

## 🚀 Real Use Case Demo: Global Invoice Financing

```bash
# 1. Alice tokenizes an unpaid $5,000 invoice
cargo run --bin finternet-cli tokenize-asset \
  --name "Invoice #12345" \
  --description "Net-30 payment from TechCorp Inc" \
  --value 5000 \
  --asset-type "invoice"

# 2. Bob views available tokenized assets
cargo run --bin finternet-cli assets

# 3. Bob purchases the invoice token for $4,500 USDC
cargo run --bin finternet-cli send-payment \
  --to <alice-wallet> \
  --amount 4500 \
  --memo "Invoice #12345 purchase"

# 4. Track the complete transaction flow
cargo run --bin finternet-cli history --limit 10
```

**Result**: Alice gets immediate working capital, Bob owns the invoice token, all transactions are auditable on Solana.

## ✅ Core MVP Functionalities (Fully Implemented)

### 1. Asset Tokenization Module ✅

- **✅ Token Schema**: Complete metadata structure for any asset type
- **✅ SPL Token Minting**: Real tokens minted on Solana using SPL Token program
- **✅ Metadata Storage**: Using Metaplex for decentralized metadata storage
- **✅ CLI Interface**: `tokenize-asset` command with full parameter support

```rust
// Tokenize any asset with one function call
let (mint_address, metadata) = client.tokenize_asset(
    "Luxury Apartment NYC",
    "2-bedroom apartment in Manhattan", 
    2_500_000, // $2.5M value
    "real_estate",
    &wallet
).await?;
```

### 2. Unified Ledger Access Layer ✅

- **✅ Asset Discovery**: Query all tokenized assets for any user
- **✅ Transaction History**: Complete on-chain transaction tracking
- **✅ Metadata Queries**: Retrieve asset metadata and ownership history
- **✅ Composable Architecture**: Built for integration with DeFi protocols

```rust
// View all assets owned by a wallet
let assets = client.get_owned_assets(&wallet_pubkey).await?;

// Get complete transaction history
let history = client.get_transaction_history(&wallet_pubkey, Some(50)).await?;
```

### 3. Cross-Border Payments Module ✅

- **✅ USDC Integration**: Send/receive USDC with proper decimal handling
- **✅ SPL Token Support**: Support for any SPL token transfers
- **✅ Transaction Receipts**: On-chain transaction hashes for settlement tracking
- **✅ Memo Support**: Attach payment descriptions and references

```rust
// Send cross-border payment with full auditability
let signature = client.send_usdc_payment(
    &sender_wallet,
    &recipient_pubkey,
    1000.0, // $1,000 USDC
    Some("Invoice #12345 payment")
).await?;
```

### 4. End-to-End Demo Flow ✅

**Working Example**: `cargo run --example basic_flow`

Demonstrates:
- ✅ Real asset tokenization on Solana
- ✅ Identity registration and management
- ✅ Asset discovery and ownership tracking
- ✅ Payment simulation with USDC
- ✅ Complete transaction history
- ✅ Unified wallet information

## 🛠️ Tech Stack

| Layer | Technology |
|-------|------------|
| **Smart Contracts** | Solana + SPL Token + Metaplex |
| **Token Standards** | SPL Token with Metaplex metadata |
| **Ledger Storage** | Solana Account Data + Metaplex |
| **SDK Interface** | Rust crate + CLI + Examples |
| **Dev Tooling** | Cargo, Solana CLI |
| **Network** | Solana Devnet (mainnet ready) |

## 🏃‍♂️ Quick Start

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Create/configure wallet
solana-keygen new
solana config set --url devnet
```

### Installation & Demo
```bash
# Clone and build
git clone <your-repo-url>
cd finternet-sdk-rust
cargo build --release

# Get devnet SOL for gas fees
solana airdrop 2

# Run the complete demo
cargo run --example basic_flow

# Try the CLI
./target/release/finternet-cli --help
```

## 📖 Usage Examples

### Asset Tokenization
```rust
use finternet_sdk::FinternetClient;

let client = FinternetClient::new_devnet();
let wallet = FinternetClient::load_default_wallet()?;

// Tokenize a real estate property
let (mint, metadata) = client.tokenize_asset(
    "Commercial Building",
    "Prime downtown office space",
    10_000_000, // $10M
    "real_estate",
    &wallet
).await?;

println!("Asset tokenized: {}", mint);
```

### Cross-Border Payments
```rust
// Send USDC payment
let signature = client.send_usdc_payment(
    &sender_wallet,
    &recipient_pubkey,
    500.0, // $500
    Some("Trade finance payment")
).await?;

// Track settlement
let status = client.get_transaction_status(&signature).await?;
println!("Payment status: {}", status);
```

### Ledger Queries
```rust
// Get all assets owned by a user
let assets = client.get_owned_assets(&user_pubkey).await?;
for (mint, balance) in assets {
    let metadata = client.get_asset_info(&mint).await?;
    println!("Asset: {} - Value: ${}", metadata.name, metadata.value);
}
```

## 🎯 Business Value

### "The Stripe for Tokenization + Payments on Finternet"

- **🔌 Plug & Play**: Add tokenization to any app with simple Rust calls
- **🌐 Cross-Border Ready**: Built-in USDC and SPL token support
- **🔒 Auditable**: All transactions tracked on Solana blockchain
- **🏗️ Composable**: Integrates with existing DeFi protocols
- **📈 Scalable**: Solana's high throughput for global finance

### Potential Integrations
- **DeFi Protocols**: Lending, yield farming with tokenized assets
- **Remittance Services**: Low-cost international transfers
- **Trade Finance**: Invoice factoring, supply chain financing
- **Real Estate**: Fractional property ownership
- **Commodities**: Gold, oil, agricultural product tokenization

## 🧪 Testing

```bash
# Run all tests
cargo test

# Check code compilation
cargo check

# Run examples
cargo run --example basic_flow

# Test CLI functionality
cargo run --bin finternet-cli balance
```

## 📋 API Reference

### Core Client
```rust
// Initialize client
let client = FinternetClient::new_devnet();
let client = FinternetClient::new(custom_config);

// Asset operations
client.tokenize_asset(name, desc, value, type, wallet).await?;
client.get_asset_info(mint_address).await?;
client.is_valid_asset(mint_address).await?;

// Payment operations  
client.send_usdc_payment(from, to, amount, memo).await?;
client.send_payment(from, to, amount, mint, memo).await?;
client.get_usdc_balance(wallet).await?;

// Ledger operations
client.get_transaction_history(wallet, limit).await?;
client.get_owned_assets(wallet).await?;
client.get_token_accounts(wallet).await?;

// Identity operations
client.register_identity(wallet, name, metadata).await?;
client.get_identity(wallet).await?;
client.get_wallet_info(wallet).await?;
```

### CLI Commands
```bash
# Asset management
finternet-cli tokenize-asset --name "Asset" --value 1000000
finternet-cli assets --address <wallet>
finternet-cli asset-info --mint <mint-address>

# Payments
finternet-cli send-payment --to <pubkey> --amount 100
finternet-cli send-token --to <pubkey> --amount 1000 --token-mint <mint>
finternet-cli balance

# Ledger access
finternet-cli history --limit 20
finternet-cli wallet-info

# Identity
finternet-cli register-identity --display-name "Alice" --email "alice@example.com"
finternet-cli create-wallet --output-path ./my-wallet.json
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎉 Acknowledgments

- Built for the Finternet Foundation MVP Grant Program
- Powered by Solana blockchain and SPL Token standards
- Metadata handling via Metaplex
- Inspired by the vision of unified global financial infrastructure

---

**Ready to build the future of tokenized finance? Start with Finternet SDK!** 🚀 