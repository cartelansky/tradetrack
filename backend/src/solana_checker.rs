#![allow(deprecated)]
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::UiTransactionEncoding;
use chrono::{DateTime, Utc, Duration};
use anyhow::{Result, anyhow};
use std::str::FromStr;

const USDC_MINT: &str = "4zddC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";
const USDT_MINT: &str = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";
const TIME_WINDOW_MINUTES: i64 = 60;

pub struct SolanaChecker {
    rpc_client: RpcClient,
    wallet_address: Pubkey,
}

impl SolanaChecker {
    pub fn new(rpc_url: &str, wallet_address: &str) -> Result<Self> {
        let rpc_client = RpcClient::new(rpc_url.to_string());
        let wallet_address = Pubkey::from_str(wallet_address)
            .map_err(|e| anyhow!("Invalid wallet address: {}", e))?;

        Ok(Self {
            rpc_client,
            wallet_address,
        })
    }

    pub async fn verify_payment(&self, signature: &str, expected_amount: f64) -> Result<bool> {
        let tx = self.rpc_client
            .get_transaction_with_config(
                &signature.parse()?,
                solana_client::rpc_config::RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::JsonParsed),
                    commitment: None,
                    max_supported_transaction_version: Some(0),
                },
            )?;

        let transaction_time = DateTime::<Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(tx.block_time.unwrap() as i64, 0).unwrap(),
            Utc,
        );

        let current_time = Utc::now();
        let time_difference = current_time.signed_duration_since(transaction_time);

        if time_difference > Duration::minutes(TIME_WINDOW_MINUTES) {
            return Ok(false);
        }

        if let Some(meta) = tx.transaction.meta {
            let pre_balances = match meta.pre_token_balances {
                solana_transaction_status::option_serializer::OptionSerializer::Some(balances) => balances,
                _ => vec![],
            };

            let post_balances = match meta.post_token_balances {
                solana_transaction_status::option_serializer::OptionSerializer::Some(balances) => balances,
                _ => vec![],
            };

            for (pre, post) in pre_balances.iter().zip(post_balances.iter()) {
                let owner = match &pre.owner {
                    solana_transaction_status::option_serializer::OptionSerializer::Some(owner_str) => owner_str,
                    _ => continue,
                };

                if owner == &self.wallet_address.to_string() {
                    let token_mint = &pre.mint;
                    let pre_amount = pre.ui_token_amount.ui_amount.unwrap_or(0.0);
                    let post_amount = post.ui_token_amount.ui_amount.unwrap_or(0.0);
                    let amount_change = (post_amount - pre_amount).abs();

                    if (token_mint == USDC_MINT || token_mint == USDT_MINT) && 
                       (amount_change - expected_amount).abs() < 0.01 {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }
} 