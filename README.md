# Finternet SDK (Rust) 🦀

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/solana-devnet-blue.svg)](https://docs.solana.com/)

A Rust-based SDK for building on the **Finternet** - enabling asset tokenization, unified ledger interactions, and cross-border payments on Solana.

## ✨ Features

- 🏭 **Asset Tokenization**: Mint SPL tokens representing real-world or digital assets with metadata
- 📚 **Unified Ledger**: Read/write transactions and asset data on-chain
- 💸 **Cross-Border Payments**: Send USDC and other SPL tokens with memo support
- 🆔 **Identity Management**: Basic wallet-based identity with on-chain registration
- 🖥️ **CLI Interface**: Complete command-line tool for all operations
- 🔧 **Modular Design**: Clean, reusable Rust modules ready for integration

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (for wallet management)

### Installation

```bash
git clone https://github.com/your-org/finternet-sdk-rust.git
cd finternet-sdk-rust
cargo build --release
```

### Setup Wallet

```bash
# Create a new wallet
solana-keygen new --outfile ~/.config/solana/id.json

# Airdrop SOL for gas fees (devnet)
solana airdrop 2

# Check balance
solana balance
```

## 🎯 Usage Examples

### Run the Demo

```bash
cargo run --example basic_flow
```

This will:
1. 🏭 Tokenize a real estate asset
2. 🔍 Read asset information from the ledger
3. 🆔 Register an identity
4. 💸 Demonstrate payment capabilities
5. 📜 Show transaction history

### CLI Commands

```bash
# Build the CLI
cargo build --bin finternet-cli

# Tokenize an asset
./target/debug/finternet-cli tokenize-asset \
  --name "Luxury Apartment" \
  --description "Downtown 2BR apartment" \
  --value 750000 \
  --asset-type "real_estate"

# Send USDC payment
./target/debug/finternet-cli send-payment \
  --to 9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM \
  --amount 10.50 \
  --memo "Invoice #1234"

# Check balances
./target/debug/finternet-cli balance

# View transaction history
./target/debug/finternet-cli history --limit 5

# Get owned assets
./target/debug/finternet-cli assets
```

### Library Usage

```rust
use finternet_sdk::{FinternetClient, FinternetConfig};
use solana_sdk::signature::Keypair;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize client
    let client = FinternetClient::new_devnet();
    
    // Load wallet
    let wallet = FinternetClient::load_default_wallet()?;
    
    // Tokenize an asset
    let (mint_address, metadata) = client
        .tokenize_asset(
            "Art Collection",
            "Rare digital art NFT collection",
            100_000,
            "digital_art",
            &wallet,
        )
        .await?;
    
    println!("Asset tokenized: {}", mint_address);
    
    // Send payment
    let recipient = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".parse()?;
    let signature = client
        .send_usdc_payment(&wallet, &recipient, 25.0, Some("Payment for services"))
        .await?;
    
    println!("Payment sent: {}", signature);
    
    Ok(())
}
```

## 📚 API Reference

### Core Client

```rust
pub struct FinternetClient {
    pub config: FinternetConfig,
    pub client: RpcClient,
}

impl FinternetClient {
    pub fn new(config: FinternetConfig) -> Self
    pub fn new_devnet() -> Self
}
```

### Asset Tokenization

```rust
impl FinternetClient {
    /// Tokenize a real-world or digital asset
    pub async fn tokenize_asset(
        &self,
        name: &str,
        description: &str,
        value: u64,
        asset_type: &str,
        wallet: &Keypair,
    ) -> Result<(Pubkey, AssetMetadata)>
    
    /// Get asset information from blockchain
    pub async fn get_asset_info(&self, token_mint: &Pubkey) -> Result<AssetMetadata>
}
```

### Payments

```rust
impl FinternetClient {
    /// Send SPL token payment
    pub async fn send_payment(
        &self,
        from_wallet: &Keypair,
        to_pubkey: &Pubkey,
        amount: u64,
        token_mint: &Pubkey,
        memo: Option<&str>,
    ) -> Result<Signature>
    
    /// Send USDC payment (convenience method)
    pub async fn send_usdc_payment(
        &self,
        from_wallet: &Keypair,
        to_pubkey: &Pubkey,
        amount_usdc: f64,
        memo: Option<&str>,
    ) -> Result<Signature>
}
```

### Ledger Operations

```rust
impl FinternetClient {
    /// Get transaction history
    pub async fn get_transaction_history(
        &self,
        owner: &Pubkey,
        limit: Option<usize>,
    ) -> Result<Vec<TransactionRecord>>
    
    /// Get owned assets
    pub async fn get_owned_assets(&self, owner: &Pubkey) -> Result<Vec<(Pubkey, u64)>>
    
    /// Write custom ledger entry
    pub async fn write_ledger_entry(
        &self,
        wallet: &Keypair,
        entry_data: &str,
    ) -> Result<Signature>
}
```

### Identity Management

```rust
impl FinternetClient {
    /// Load wallet from default Solana CLI location
    pub fn load_default_wallet() -> Result<Keypair>
    
    /// Create new wallet
    pub fn create_new_wallet() -> Keypair
    
    /// Register identity on-chain
    pub async fn register_identity(
        &self,
        wallet: &Keypair,
        display_name: &str,
        metadata: HashMap<String, String>,
    ) -> Result<Signature>
    
    /// Get identity information
    pub async fn get_identity(&self, pubkey: &Pubkey) -> Result<FinternetIdentity>
}
```

## 🏗️ Architecture

```
finternet-sdk/
├── src/
│   ├── lib.rs          # Main library exports
│   ├── asset.rs        # Asset tokenization module
│   ├── payment.rs      # Cross-border payments
│   ├── ledger.rs       # Unified ledger interactions
│   ├── identity.rs     # Identity management
│   └── cli.rs          # Command-line interface
├── examples/
│   └── basic_flow.rs   # End-to-end demo
├── Cargo.toml          # Dependencies and metadata
└── README.md           # This file
```

## 🔧 Configuration

### Environment Variables

```bash
# Optional: Custom RPC endpoint
export SOLANA_RPC_URL="https://api.devnet.solana.com"

# Optional: Custom wallet path
export SOLANA_WALLET_PATH="~/.config/solana/id.json"
```

### Network Support

- **Devnet** (default): Full testing environment
- **Mainnet**: Production environment (update USDC mint addresses)
- **Localnet**: Local Solana validator

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=info cargo test

# Test specific module
cargo test asset::tests
```

## 🚧 Roadmap

- [ ] **Enhanced Metadata**: IPFS integration for richer asset metadata
- [ ] **DID Integration**: Full Decentralized Identity support
- [ ] **Multi-sig Support**: Corporate account management
- [ ] **Custom Programs**: Deploy specialized Solana programs
- [ ] **WebAssembly**: Compile to WASM for web integration
- [ ] **Mobile SDK**: React Native bindings
- [ ] **Advanced Analytics**: Portfolio tracking and reporting

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📖 Use Cases

### Real Estate Tokenization
```bash
finternet-cli tokenize-asset \
  --name "Miami Beach Condo" \
  --description "Luxury oceanfront property" \
  --value 2500000 \
  --asset-type "real_estate"
```

### Cross-Border Remittance
```bash
finternet-cli send-payment \
  --to 9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM \
  --amount 500.00 \
  --memo "Family support payment"
```

### Supply Chain Finance
```bash
# Tokenize inventory
finternet-cli tokenize-asset \
  --name "Coffee Shipment #1234" \
  --description "Premium arabica beans from Colombia" \
  --value 45000 \
  --asset-type "commodity"

# Transfer ownership
finternet-cli send-token \
  --to BuyerWalletAddress... \
  --amount 1 \
  --token-mint CoffeeTokenMint... \
  --memo "Ownership transfer upon delivery"
```

## 🔐 Security

- **Wallet Security**: Never share private keys
- **Devnet Only**: This MVP is designed for devnet testing
- **Audit Pending**: Production use requires security audit
- **Best Practices**: Follow Solana security guidelines

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- 📖 [Documentation](https://docs.finternet.org)
- 💬 [Discord Community](https://discord.gg/finternet)
- 🐛 [Issues](https://github.com/your-org/finternet-sdk-rust/issues)
- 📧 [Email Support](mailto:support@finternet.org)

## 🙏 Acknowledgments

- [Solana Labs](https://solana.com/) for the blockchain infrastructure
- [Metaplex](https://www.metaplex.com/) for NFT metadata standards
- The Rust community for excellent tooling and libraries

---

**🚀 Ready to build the future of finance? Start tokenizing assets today!** 