use crate::{FinternetClient, TransactionRecord};
use anyhow::Result;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Signature, Signer},
    program_pack::Pack,
};
use solana_transaction_status::{
    UiTransactionEncoding, EncodedConfirmedTransactionWithStatusMeta,
    option_serializer::OptionSerializer
};
use solana_account_decoder::UiAccountData;
use std::collections::HashMap;

impl FinternetClient {
    /// Get transaction history for a given wallet address
    pub async fn get_transaction_history(
        &self,
        owner: &Pubkey,
        limit: Option<usize>,
    ) -> Result<Vec<TransactionRecord>> {
        let limit = limit.unwrap_or(10);
        log::info!("Fetching transaction history for: {} (limit: {})", owner, limit);
        
        // Get recent signatures for the account
        let signatures = self.client.get_signatures_for_address_with_config(
            owner,
            GetConfirmedSignaturesForAddress2Config {
                before: None,
                until: None,
                limit: Some(limit),
                commitment: Some(CommitmentConfig::confirmed()),
            },
        )?;
        
        let mut transaction_records = Vec::new();
        
        // Process up to the limit of transactions
        for sig_info in signatures.iter().take(limit) {
            let signature: Signature = sig_info.signature.parse()?;
            
            // Get transaction details
            if let Ok(transaction) = self.client.get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Json),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            ) {
                if let Some(meta) = &transaction.transaction.meta {
                    // Extract token transfers from the transaction
                    if let OptionSerializer::Some(ref pre_token_balances) = meta.pre_token_balances {
                        if let OptionSerializer::Some(ref post_token_balances) = meta.post_token_balances {
                            // Match pre and post balances to find transfers
                            for (pre_balance, post_balance) in 
                                pre_token_balances.iter().zip(post_token_balances.iter()) {
                                
                                if pre_balance.account_index == post_balance.account_index {
                                    let pre_amount = pre_balance.ui_token_amount.amount.parse::<u64>().unwrap_or(0);
                                    let post_amount = post_balance.ui_token_amount.amount.parse::<u64>().unwrap_or(0);
                                    
                                    if pre_amount != post_amount {
                                        // Determine if this is a send or receive
                                        let amount = if post_amount > pre_amount {
                                            post_amount - pre_amount
                                        } else {
                                            pre_amount - post_amount
                                        };
                                        
                                        let token_mint = pre_balance.mint.parse()?;
                                        
                                        // Extract memo if present
                                        let memo = self.extract_memo_from_transaction(&transaction);
                                        
                                        let record = TransactionRecord {
                                            signature,
                                            from: *owner, // Simplified - would need more logic to determine actual from/to
                                            to: *owner,
                                            amount,
                                            token_mint,
                                            timestamp: sig_info.block_time.unwrap_or(0) as u64,
                                            memo,
                                        };
                                        
                                        transaction_records.push(record);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        log::info!("Found {} transaction records", transaction_records.len());
        Ok(transaction_records)
    }
    
    /// Get detailed transaction information by signature
    pub async fn get_transaction_details(&self, signature: &Signature) -> Result<Option<TransactionRecord>> {
        log::info!("Fetching transaction details for: {}", signature);
        
        let transaction = self.client.get_transaction_with_config(
            signature,
            RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Json),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: Some(0),
            },
        )?;
        
        // Extract transaction details
        if let Some(meta) = &transaction.transaction.meta {
            if let OptionSerializer::Some(ref pre_token_balances) = meta.pre_token_balances {
                if let OptionSerializer::Some(ref post_token_balances) = meta.post_token_balances {
                    for (pre_balance, post_balance) in 
                        pre_token_balances.iter().zip(post_token_balances.iter()) {
                        
                        if pre_balance.account_index == post_balance.account_index {
                            let pre_amount = pre_balance.ui_token_amount.amount.parse::<u64>().unwrap_or(0);
                            let post_amount = post_balance.ui_token_amount.amount.parse::<u64>().unwrap_or(0);
                            
                            if pre_amount != post_amount {
                                let amount = if post_amount > pre_amount {
                                    post_amount - pre_amount
                                } else {
                                    pre_amount - post_amount
                                };
                                
                                let token_mint = pre_balance.mint.parse()?;
                                let memo = self.extract_memo_from_transaction(&transaction);
                                
                                let record = TransactionRecord {
                                    signature: *signature,
                                    from: Pubkey::default(), // Would need more complex logic
                                    to: Pubkey::default(),
                                    amount,
                                    token_mint,
                                    timestamp: transaction.block_time.unwrap_or(0) as u64,
                                    memo,
                                };
                                
                                return Ok(Some(record));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    /// Get all token accounts owned by a wallet
    pub async fn get_token_accounts(&self, owner: &Pubkey) -> Result<HashMap<Pubkey, u64>> {
        log::info!("Fetching token accounts for: {}", owner);
        
        let token_accounts = self.client.get_token_accounts_by_owner(
            owner,
            TokenAccountsFilter::ProgramId(spl_token::id()),
        )?;
        
        log::info!("Raw RPC response: {} token accounts found", token_accounts.len());
        
        let mut balances = HashMap::new();
        
        for (i, account) in token_accounts.iter().enumerate() {
            log::debug!("Processing account {}: pubkey={}", i, account.pubkey);
            log::debug!("Account data type: {:?}", account.account.data);
            
            // Decode the account data properly - handle both Binary and JSON formats
            match &account.account.data {
                UiAccountData::Binary(data, encoding) => {
                    log::debug!("Account {} has encoding: {:?}, data length: {}", i, encoding, data.len());
                    
                    let decoded_data = match encoding {
                        solana_account_decoder::UiAccountEncoding::Base64 => {
                            // Use the modern base64 engine instead of deprecated function
                            use base64::{Engine, engine::general_purpose};
                            match general_purpose::STANDARD.decode(data) {
                                Ok(decoded) => {
                                    log::debug!("Successfully decoded base64 data, length: {}", decoded.len());
                                    decoded
                                }
                                Err(e) => {
                                    log::warn!("Failed to decode base64 for account {}: {}", i, e);
                                    continue;
                                }
                            }
                        }
                        solana_account_decoder::UiAccountEncoding::Base58 => {
                            match bs58::decode(data).into_vec() {
                                Ok(decoded) => {
                                    log::debug!("Successfully decoded base58 data, length: {}", decoded.len());
                                    decoded
                                }
                                Err(e) => {
                                    log::warn!("Failed to decode base58 for account {}: {}", i, e);
                                    continue;
                                }
                            }
                        }
                        _ => {
                            log::warn!("Skipping account {} with unsupported encoding: {:?}", i, encoding);
                            continue;
                        }
                    };
                    
                    match spl_token::state::Account::unpack(&decoded_data) {
                        Ok(token_account) => {
                            log::info!("✅ Successfully unpacked token account {}: mint={}, amount={}", 
                                      i, token_account.mint, token_account.amount);
                            balances.insert(token_account.mint, token_account.amount);
                        }
                        Err(e) => {
                            log::warn!("Failed to unpack token account {}: {}", i, e);
                        }
                    }
                }
                UiAccountData::Json(parsed_account) => {
                    log::debug!("Account {} is in JSON format", i);
                    
                    // Handle JSON parsed account data
                    let parsed = &parsed_account.parsed;
                    if let Some(info) = parsed.get("info") {
                        // Extract mint and token amount from JSON
                        if let (Some(mint_str), Some(token_amount)) = (
                            info.get("mint").and_then(|v| v.as_str()),
                            info.get("tokenAmount")
                        ) {
                            if let Some(amount_str) = token_amount.get("amount").and_then(|v| v.as_str()) {
                                match (mint_str.parse::<Pubkey>(), amount_str.parse::<u64>()) {
                                    (Ok(mint), Ok(amount)) => {
                                        log::info!("✅ Successfully parsed JSON token account {}: mint={}, amount={}", 
                                                  i, mint, amount);
                                        balances.insert(mint, amount);
                                    }
                                    (Err(e), _) => {
                                        log::warn!("Failed to parse mint for account {}: {}", i, e);
                                    }
                                    (_, Err(e)) => {
                                        log::warn!("Failed to parse amount for account {}: {}", i, e);
                                    }
                                }
                            } else {
                                log::warn!("No amount found in tokenAmount for account {}", i);
                            }
                        } else {
                            log::warn!("Missing mint or tokenAmount in account {} info", i);
                        }
                    } else {
                        log::warn!("No info field in parsed account {}", i);
                    }
                }
                _ => {
                    log::debug!("Account {} data format not supported", i);
                }
            }
        }
        
        log::info!("Successfully found {} token accounts with balances", balances.len());
        Ok(balances)
    }
    
    /// Get all assets (tokens) owned by a wallet with their metadata
    pub async fn get_owned_assets(&self, owner: &Pubkey) -> Result<Vec<(Pubkey, u64)>> {
        log::info!("Fetching owned assets for: {}", owner);
        
        let token_accounts = self.get_token_accounts(owner).await?;
        let mut assets = Vec::new();
        
        for (mint, balance) in token_accounts {
            // Only include tokens where the user has a balance > 0
            if balance > 0 {
                assets.push((mint, balance));
            }
        }
        
        log::info!("Found {} owned assets", assets.len());
        Ok(assets)
    }
    
    /// Check the status of a transaction
    pub async fn get_transaction_status(&self, signature: &Signature) -> Result<String> {
        match self.client.get_signature_status(signature)? {
            Some(status) => {
                match status {
                    Ok(_) => Ok("Confirmed".to_string()),
                    Err(e) => Ok(format!("Failed: {}", e)),
                }
            }
            None => Ok("Pending".to_string()),
        }
    }
    
    /// Get the current slot and block time (for timestamping)
    pub async fn get_current_slot_and_time(&self) -> Result<(u64, u64)> {
        let slot = self.client.get_slot()?;
        let block_time = self.client.get_block_time(slot)? as u64;
        
        Ok((slot, block_time))
    }
    
    /// Write a custom log entry to the ledger (using a memo transaction)
    pub async fn write_ledger_entry(
        &self,
        wallet: &solana_sdk::signature::Keypair,
        entry_data: &str,
    ) -> Result<Signature> {
        log::info!("Writing ledger entry: {}", entry_data);
        
        let memo_ix = solana_sdk::instruction::Instruction {
            program_id: "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr".parse()?,
            accounts: vec![solana_sdk::instruction::AccountMeta::new_readonly(
                wallet.pubkey(),
                true,
            )],
            data: entry_data.as_bytes().to_vec(),
        };
        
        let recent_blockhash = self.client.get_latest_blockhash()?;
        let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
            &[memo_ix],
            Some(&wallet.pubkey()),
        );
        transaction.sign(&[wallet], recent_blockhash);
        
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        log::info!("Ledger entry written with signature: {}", signature);
        
        Ok(signature)
    }
    
    /// Helper function to extract memo from transaction
    fn extract_memo_from_transaction(
        &self,
        _transaction: &EncodedConfirmedTransactionWithStatusMeta,
    ) -> Option<String> {
        // Simplified memo extraction - for now just return None
        // In a full implementation, we would parse the transaction instructions
        // to find memo program calls, but this requires complex parsing
        None
    }
    
    /// Request devnet USDC airdrop for testing
    pub async fn request_devnet_usdc(&self, wallet_pubkey: &Pubkey) -> Result<String> {
        use crate::payment::usdc;
        
        // For devnet, we'll create an associated token account and show how to get USDC
        let usdc_mint = usdc::devnet_mint();
        let ata = spl_associated_token_account::get_associated_token_address(
            wallet_pubkey,
            &usdc_mint,
        );
        
        log::info!("USDC associated token account for {}: {}", wallet_pubkey, ata);
        
        // Check if ATA exists
        match self.client.get_token_account_balance(&ata) {
            Ok(balance) => {
                Ok(format!("USDC ATA exists with balance: {}", balance.ui_amount_string))
            }
            Err(_) => {
                Ok(format!(
                    "USDC ATA needs to be created: {}. Use `spl-token create-account {}` or fund it via a faucet service.",
                    ata,
                    usdc_mint
                ))
            }
        }
    }
    
    /// Enhanced asset discovery that includes all token accounts
    pub async fn discover_all_tokens(&self, wallet_pubkey: &Pubkey) -> Result<Vec<(Pubkey, u64, Option<String>)>> {
        log::info!("Starting enhanced token discovery for: {}", wallet_pubkey);
        
        let mut discovered_tokens = Vec::new();
        
        // Use our enhanced get_token_accounts method
        let token_accounts = self.get_token_accounts(wallet_pubkey).await?;
        
        log::info!("Processing {} token accounts for metadata", token_accounts.len());
        
        for (mint, balance) in token_accounts {
            if balance > 0 {
                log::debug!("Processing token: mint={}, balance={}", mint, balance);
                
                // Try to get metadata for this token
                let metadata_name = match self.get_asset_info(&mint).await {
                    Ok(metadata) => {
                        log::debug!("Found metadata for {}: {}", mint, metadata.name);
                        Some(metadata.name)
                    }
                    Err(e) => {
                        log::debug!("No metadata found for {}: {}", mint, e);
                        None
                    }
                };
                
                discovered_tokens.push((mint, balance, metadata_name));
            }
        }
        
        log::info!("Discovery complete: found {} tokens with positive balances", discovered_tokens.len());
        Ok(discovered_tokens)
    }
    
    /// Get SOL balance for a wallet (returns amount in SOL, not lamports)
    pub async fn get_sol_balance(&self, wallet_pubkey: &Pubkey) -> Result<f64> {
        let balance_lamports = self.client.get_balance(wallet_pubkey)?;
        Ok(balance_lamports as f64 / 1_000_000_000.0)
    }
} 