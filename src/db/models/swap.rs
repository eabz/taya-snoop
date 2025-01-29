use alloy::primitives::{Log, LogData};
use serde::{Deserialize, Serialize};

use super::pair::Swap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSwap {
    pub id: String,
    pub transaction: String,
    pub timestamp: u64,
    pub pair: String,
    pub sender: String,
    pub from: String,
    pub amount0_in: f64,
    pub amount1_in: f64,
    pub amount0_out: f64,
    pub amount1_out: f64,
    pub to: String,
    pub log_index: Option<u64>,
    pub amount_usd: f64,
}

impl DatabaseSwap {
    pub fn from_log(
        log: &alloy::rpc::types::Log<LogData>,
        event: Log<Swap>,
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
            sender: event.sender.to_string(),
            // TODO: fix 'from' and 'amounts'
            from: "".to_owned(),
            amount0_in: 0.0,
            amount1_in: 0.0,
            amount0_out: 0.0,
            amount1_out: 0.0,
            to: event.to.to_string(),
            log_index: log.log_index,
            // TODO: add amount_usd
            amount_usd: 0.0,
        }
    }
}
