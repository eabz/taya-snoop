use alloy::primitives::{Log, LogData};
use bigdecimal::{BigDecimal, Zero};
use serde::{Deserialize, Serialize};

use super::pair::Mint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMint {
    pub id: String,
    pub transaction: String,
    pub timestamp: u64,
    pub pair: String,
    pub to: String,
    pub liquidity: BigDecimal,
    pub sender: Option<String>,
    pub amount0: Option<BigDecimal>,
    pub amount1: Option<BigDecimal>,
    pub log_index: Option<u64>,
    pub amount_usd: Option<BigDecimal>,
    pub fee_to: Option<String>,
    pub fee_liquidity: Option<BigDecimal>,
}

impl DatabaseMint {
    pub fn from_log(
        log: &alloy::rpc::types::Log<LogData>,
        event: Log<Mint>,
    ) -> Self {
        let transaction = log.transaction_hash.unwrap().to_string();

        Self {
            id: format!(
                "{}-{}",
                transaction,
                log.transaction_index.unwrap()
            ),
            transaction,
            timestamp: log.block_timestamp.unwrap(),
            pair: event.address.to_string(),
            // TODO: fix 'to' and 'liquidity'
            to: "".to_owned(),
            liquidity: BigDecimal::zero(),
            sender: Some(event.sender.to_string()),
            // TODO: fix amounts
            amount0: Some(BigDecimal::zero()),
            amount1: Some(BigDecimal::zero()),
            log_index: log.log_index,
            // TODO: fix amount usd and fees
            amount_usd: Some(BigDecimal::zero()),
            fee_to: Some("".to_owned()),
            fee_liquidity: Some(BigDecimal::zero()),
        }
    }
}
