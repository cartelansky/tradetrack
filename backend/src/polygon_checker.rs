#![allow(unused_imports, deprecated)]
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};
use std::str::FromStr;
use web3::types::{BlockNumber, H160, H256, U256};
use web3::Web3;

const USDT_CONTRACT: &str = "0x4aE94Eb019C0762f9Bfcf9Fb1E58725BfB0e7582";
const USDC_CONTRACT: &str = "0x41E94Eb019C0762f9Bfcf9Fb1E58725BfB0e7582";
const TRANSFER_EVENT_SIGNATURE: &str = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
const TIME_WINDOW_MINUTES: i64 = 60;

pub struct PolygonChecker {
    web3: Web3<web3::transports::Http>,
    wallet_address: H160,
}

impl PolygonChecker {
    pub fn new(rpc_url: &str, wallet_address: &str) -> Result<Self> {
        let transport = web3::transports::Http::new(rpc_url)?;
        let web3 = Web3::new(transport);
        let wallet_address = H160::from_str(wallet_address)
            .map_err(|e| anyhow!("Invalid wallet address: {}", e))?;

        Ok(Self {
            web3,
            wallet_address,
        })
    }

    pub async fn verify_payment(&self, tx_hash: &str, expected_amount: f64) -> Result<bool> {
        let tx_hash = H256::from_str(tx_hash)
            .map_err(|e| anyhow!("Invalid transaction hash: {}", e))?;

        // Get transaction details
        let tx = self.web3.eth().transaction(tx_hash.into()).await?
            .ok_or_else(|| anyhow!("Transaction not found"))?;

        // Get block details
        let block = self.web3.eth().block(tx.block_hash.unwrap().into()).await?
            .ok_or_else(|| anyhow!("Block not found"))?;

        // Check transaction time
        let block_time = DateTime::<Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(block.timestamp.as_u64() as i64, 0).unwrap(),
            Utc,
        );

        if Utc::now().signed_duration_since(block_time) > Duration::minutes(TIME_WINDOW_MINUTES) {
            return Ok(false);
        }

        // Convert expected amount to token units (6 decimals for USDC/USDT)
        let expected_amount_wei = (expected_amount * 1_000_000.0) as u128;

        // Check token transfers
        for &token_address in &[USDT_CONTRACT, USDC_CONTRACT] {
            let token_contract = H160::from_str(token_address)?;

            let mut padded_address = [0u8; 32];
            padded_address[12..32].copy_from_slice(self.wallet_address.as_fixed_bytes());
            let topic2 = H256::from(padded_address);

            let logs = self.web3.eth().logs(
                web3::types::FilterBuilder::default()
                    .from_block(BlockNumber::Number(block.number.unwrap()))
                    .to_block(BlockNumber::Number(block.number.unwrap()))
                    .address(vec![token_contract])
                    .topics(
                        Some(vec![H256::from_str(TRANSFER_EVENT_SIGNATURE)?]),
                        None,
                        Some(vec![topic2]),
                        None,
                    )
                    .build(),
            ).await?;

            for log in logs {
                if log.transaction_hash.unwrap() == tx_hash {
                    let amount = U256::from_big_endian(&log.data.0);
                    let amount_u128 = amount.as_u128();
                    
                    // Allow small difference in amount (0.01 tokens)
                    if (amount_u128 as f64 - expected_amount_wei as f64).abs() < 10_000.0 {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }
} 