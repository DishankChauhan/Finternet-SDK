use crate::{AssetMetadata, FinternetClient};
use anyhow::Result;
use mpl_token_metadata::{
    accounts::Metadata,
    instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs},
    types::{Creator, DataV2},
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::instruction as ata_instruction;
use spl_token::instruction as token_instruction;
use std::time::{SystemTime, UNIX_EPOCH};

impl FinternetClient {
    /// Tokenize a real-world or digital asset by minting an SPL token with metadata
    pub async fn tokenize_asset(
        &self,
        name: &str,
        description: &str,
        value: u64,
        asset_type: &str,
        wallet: &Keypair,
    ) -> Result<(Pubkey, AssetMetadata)> {
        log::info!(
            "Tokenizing asset: {} of type: {} with value: {}",
            name,
            asset_type,
            value
        );

        // Create a new mint keypair
        let mint_keypair = Keypair::new();
        let mint_pubkey = mint_keypair.pubkey();
        
        // Get recent blockhash
        let recent_blockhash = self.client.get_latest_blockhash()?;
        
        // Calculate rent exemption for mint account
        let mint_rent = self.client.get_minimum_balance_for_rent_exemption(82)?; // 82 bytes for mint account
        
        // Create mint account instruction
        let create_mint_account_ix = system_instruction::create_account(
            &wallet.pubkey(),
            &mint_pubkey,
            mint_rent,
            82,
            &spl_token::id(),
        );
        
        // Initialize mint instruction
        let init_mint_ix = token_instruction::initialize_mint(
            &spl_token::id(),
            &mint_pubkey,
            &wallet.pubkey(),
            Some(&wallet.pubkey()),
            0, // decimals
        )?;
        
        // Get associated token account for the wallet
        let associated_token_account = spl_associated_token_account::get_associated_token_address(
            &wallet.pubkey(),
            &mint_pubkey,
        );
        
        // Create associated token account instruction
        let create_ata_ix = ata_instruction::create_associated_token_account(
            &wallet.pubkey(),
            &wallet.pubkey(),
            &mint_pubkey,
            &spl_token::id(),
        );
        
        // Mint 1 token to the associated token account
        let mint_to_ix = token_instruction::mint_to(
            &spl_token::id(),
            &mint_pubkey,
            &associated_token_account,
            &wallet.pubkey(),
            &[&wallet.pubkey()],
            1,
        )?;
        
        // Create metadata account
        let metadata_account = Metadata::find_pda(&mint_pubkey).0;
        
        // Create metadata instruction
        let creators = vec![Creator {
            address: wallet.pubkey(),
            verified: true,
            share: 100,
        }];
        
        let data = DataV2 {
            name: name.to_string(),
            symbol: "FINT".to_string(),
            uri: format!("https://api.finternet.com/metadata/{}", mint_pubkey),
            seller_fee_basis_points: 0,
            creators: Some(creators),
            collection: None,
            uses: None,
        };
        
        let create_metadata_ix = CreateMetadataAccountV3 {
            metadata: metadata_account,
            mint: mint_pubkey,
            mint_authority: wallet.pubkey(),
            payer: wallet.pubkey(),
            update_authority: (wallet.pubkey(), true),
            system_program: solana_sdk::system_program::id(),
            rent: None,
        }.instruction(CreateMetadataAccountV3InstructionArgs {
            data,
            is_mutable: true,
            collection_details: None,
        });
        
        // Build and send transaction
        let instructions = vec![
            create_mint_account_ix,
            init_mint_ix,
            create_ata_ix,
            mint_to_ix,
            create_metadata_ix,
        ];
        
        let mut transaction = Transaction::new_with_payer(&instructions, Some(&wallet.pubkey()));
        transaction.sign(&[wallet, &mint_keypair], recent_blockhash);
        
        let signature = self.client.send_and_confirm_transaction(&transaction)?;
        
        log::info!(
            "Asset tokenized successfully! Mint: {}, Signature: {}",
            mint_pubkey,
            signature
        );
        
        // Create asset metadata
        let asset_metadata = AssetMetadata {
            name: name.to_string(),
            description: description.to_string(),
            value,
            issuer: wallet.pubkey(),
            asset_type: asset_type.to_string(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            token_mint: Some(mint_pubkey),
        };
        
        Ok((mint_pubkey, asset_metadata))
    }
    
    /// Get asset information from the blockchain
    pub async fn get_asset_info(&self, token_mint: &Pubkey) -> Result<AssetMetadata> {
        log::info!("Fetching asset info for mint: {}", token_mint);
        
        // Get metadata account
        let metadata_account = Metadata::find_pda(token_mint).0;
        
        // Fetch metadata account data
        let metadata_account_data = self.client.get_account_data(&metadata_account)?;
        let metadata = Metadata::from_bytes(&metadata_account_data)?;
        
        // Extract creator (issuer) information - directly access metadata fields
        let issuer = metadata
            .creators
            .and_then(|creators| creators.first().map(|c| c.address))
            .unwrap_or_default();
        
        let asset_metadata = AssetMetadata {
            name: metadata.name.trim_matches('\0').to_string(),
            description: "Asset tokenized on Finternet".to_string(), // Placeholder as description isn't stored in metadata
            value: 0, // Would need to be stored in custom program data
            issuer,
            asset_type: "tokenized_asset".to_string(),
            created_at: 0, // Would need to be stored in custom program data
            token_mint: Some(*token_mint),
        };
        
        log::info!("Asset info retrieved: {:?}", asset_metadata);
        Ok(asset_metadata)
    }
    
    /// Check if a mint account exists and is valid
    pub async fn is_valid_asset(&self, token_mint: &Pubkey) -> Result<bool> {
        match self.client.get_account(token_mint) {
            Ok(account) => {
                // Check if it's a mint account owned by the token program
                Ok(account.owner == spl_token::id() && account.data.len() == 82)
            }
            Err(_) => Ok(false),
        }
    }
} 