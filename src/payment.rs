use crate::{FinternetClient, TransactionRecord};
use anyhow::Result;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use spl_associated_token_account::instruction as ata_instruction;
use spl_token::instruction as token_instruction;
use std::time::{SystemTime, UNIX_EPOCH};

/// Common USDC mint addresses for different networks
pub mod usdc {
    use solana_sdk::pubkey::Pubkey;
    
    // USDC mint on devnet
    pub const DEVNET: &str = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";
    
    // USDC mint on mainnet
    pub const MAINNET: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    
    pub fn devnet_mint() -> Pubkey {
        DEVNET.parse().unwrap()
    }
    
    pub fn mainnet_mint() -> Pubkey {
        MAINNET.parse().unwrap()
    }
}

impl FinternetClient {
    /// Send SPL token payment (e.g., USDC) between wallets
    pub async fn send_payment(
        &self,
        from_wallet: &Keypair,
        to_pubkey: &Pubkey,
        amount: u64,
        token_mint: &Pubkey,
        memo: Option<&str>,
    ) -> Result<Signature> {
        log::info!(
            "Sending payment: {} tokens from {} to {} (mint: {})",
            amount,
            from_wallet.pubkey(),
            to_pubkey,
            token_mint
        );
        
        // Get source associated token account
        let from_ata = spl_associated_token_account::get_associated_token_address(
            &from_wallet.pubkey(),
            token_mint,
        );
        
        // Get destination associated token account
        let to_ata = spl_associated_token_account::get_associated_token_address(
            to_pubkey,
            token_mint,
        );
        
        let mut instructions = Vec::new();
        
        // Check if destination ATA exists, create if not
        if self.client.get_account(&to_ata).is_err() {
            log::info!("Creating associated token account for recipient");
            let create_ata_ix = ata_instruction::create_associated_token_account(
                &from_wallet.pubkey(),
                to_pubkey,
                token_mint,
                &spl_token::id(),
            );
            instructions.push(create_ata_ix);
        }
        
        // Create transfer instruction
        let transfer_ix = token_instruction::transfer(
            &spl_token::id(),
            &from_ata,
            &to_ata,
            &from_wallet.pubkey(),
            &[&from_wallet.pubkey()],
            amount,
        )?;
        instructions.push(transfer_ix);
        
        // Add memo instruction if provided
        if let Some(memo_text) = memo {
            let memo_ix = spl_memo::build_memo(memo_text.as_bytes(), &[&from_wallet.pubkey()]);
            instructions.push(memo_ix);
        }
        
        // Get recent blockhash and build transaction
        let recent_blockhash = self.client.get_latest_blockhash()?;
        let mut transaction = Transaction::new_with_payer(&instructions, Some(&from_wallet.pubkey()));
        transaction.sign(&[from_wallet], recent_blockhash);
        
        // Send and confirm transaction
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        
        log::info!("Payment sent successfully! Signature: {}", signature);
        Ok(signature)
    }
    
    /// Send USDC payment using the devnet USDC mint
    pub async fn send_usdc_payment(
        &self,
        from_wallet: &Keypair,
        to_pubkey: &Pubkey,
        amount_usdc: f64, // Amount in USDC (e.g., 10.50)
        memo: Option<&str>,
    ) -> Result<Signature> {
        // Convert USDC amount to lamports (USDC has 6 decimals)
        let amount_lamports = (amount_usdc * 1_000_000.0) as u64;
        
        self.send_payment(
            from_wallet,
            to_pubkey,
            amount_lamports,
            &usdc::devnet_mint(),
            memo,
        ).await
    }
    
    /// Get token balance for a wallet
    pub async fn get_token_balance(
        &self,
        wallet_pubkey: &Pubkey,
        token_mint: &Pubkey,
    ) -> Result<u64> {
        let ata = spl_associated_token_account::get_associated_token_address(
            wallet_pubkey,
            token_mint,
        );
        
        match self.client.get_token_account_balance(&ata) {
            Ok(balance) => {
                let amount = balance.amount.parse::<u64>().unwrap_or(0);
                log::info!(
                    "Token balance for {} (mint: {}): {}",
                    wallet_pubkey,
                    token_mint,
                    amount
                );
                Ok(amount)
            }
            Err(_) => {
                log::warn!(
                    "No token account found for {} with mint {}",
                    wallet_pubkey,
                    token_mint
                );
                Ok(0)
            }
        }
    }
    
    /// Get USDC balance for a wallet (returns amount in USDC, not lamports)
    pub async fn get_usdc_balance(&self, wallet_pubkey: &Pubkey) -> Result<f64> {
        let balance_lamports = self.get_token_balance(wallet_pubkey, &usdc::devnet_mint()).await?;
        Ok(balance_lamports as f64 / 1_000_000.0)
    }
    
    /// Create a transaction record from a payment
    pub fn create_transaction_record(
        &self,
        signature: Signature,
        from: Pubkey,
        to: Pubkey,
        amount: u64,
        token_mint: Pubkey,
        memo: Option<String>,
    ) -> TransactionRecord {
        TransactionRecord {
            signature,
            from,
            to,
            amount,
            token_mint,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            memo,
        }
    }
    
    /// Check if a wallet has sufficient balance for a payment
    pub async fn can_afford_payment(
        &self,
        wallet_pubkey: &Pubkey,
        amount: u64,
        token_mint: &Pubkey,
    ) -> Result<bool> {
        let balance = self.get_token_balance(wallet_pubkey, token_mint).await?;
        Ok(balance >= amount)
    }
}

// Include memo program for transaction memos
mod spl_memo {
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };
    
    pub fn build_memo(memo: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
        Instruction {
            program_id: memo_program_id(),
            accounts: signer_pubkeys
                .iter()
                .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
                .collect(),
            data: memo.to_vec(),
        }
    }
    
    fn memo_program_id() -> Pubkey {
        "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
            .parse()
            .unwrap()
    }
} 