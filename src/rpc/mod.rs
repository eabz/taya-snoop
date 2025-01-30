use std::str::FromStr;

use log::info;

use crate::{
    abi::{erc20::ERC20, factory::FACTORY},
    chains::Chain,
    configs::Config,
    db::models::factory::PairCreated,
    handlers::{
        burn::Burn, mint::Mint, swap::Swap, sync::Sync, transfer::Transfer,
    },
};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::Address,
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::{Filter, Log},
    sol_types::SolEvent,
    transports::http::{Client, Http},
};

#[derive(Debug, Clone)]
pub struct Rpc {
    pub chain: Chain,
    pub client: RootProvider<Http<Client>>,
}

impl Rpc {
    pub async fn new(config: &Config) -> Self {
        info!("Starting rpc service");

        let client = ProviderBuilder::new()
            .on_http(config.rpc.clone().parse().unwrap());

        let client_id = client.get_chain_id().await;

        match client_id {
            Ok(value) => {
                if value != config.chain.id {
                    panic!("RPC chain id is invalid");
                }
            }
            Err(_) => panic!("unable to request eth_chainId"),
        }

        Self { chain: config.chain.clone(), client }
    }

    pub async fn get_last_block(&self) -> u64 {
        self.client
            .get_block_number()
            .await
            .expect("unable to get last block from RPC")
    }

    pub async fn get_factory_logs_batch(
        &self,
        first_block: u64,
        last_block: u64,
        config: &Config,
    ) -> Vec<Log> {
        let filter = Filter::new()
            .from_block(BlockNumberOrTag::Number(first_block))
            .to_block(BlockNumberOrTag::Number(last_block))
            .address(config.factory.address)
            .event(PairCreated::SIGNATURE);

        self.client
            .get_logs(&filter)
            .await
            .expect("unable to get logs from RPC")
    }

    pub async fn get_pairs_logs_batch(
        &self,
        pairs: &[String],
        first_block: u64,
        last_block: u64,
    ) -> Vec<Log> {
        let address_pairs: Vec<Address> = pairs
            .iter()
            .map(|pair| Address::from_str(pair).unwrap())
            .collect();

        let filter = Filter::new()
            .from_block(BlockNumberOrTag::Number(first_block))
            .to_block(BlockNumberOrTag::Number(last_block))
            .address(address_pairs)
            .events(vec![
                Mint::SIGNATURE,
                Burn::SIGNATURE,
                Swap::SIGNATURE,
                Sync::SIGNATURE,
                Transfer::SIGNATURE,
            ]);

        self.client
            .get_logs(&filter)
            .await
            .expect("unable to get logs from RPC")
    }

    pub async fn get_token_information(
        &self,
        token: String,
    ) -> (String, String, String, i64) {
        let token =
            ERC20::new(Address::from_str(&token).unwrap(), &self.client);

        let name = match token.name().call().await {
            Ok(name) => name._0,
            Err(_) => "UNKNOWN".to_owned(),
        };

        let symbol = match token.symbol().call().await {
            Ok(symbol) => symbol._0,
            Err(_) => "UNKNOWN".to_owned(),
        };

        let total_supply = match token.totalSupply().call().await {
            Ok(total_supply) => total_supply._0.to_string(),
            Err(_) => "0".to_owned(),
        };

        let decimals = match token.decimals().call().await {
            Ok(decimals) => decimals._0,
            Err(_) => 0,
        };

        (name, symbol, total_supply, decimals as i64)
    }

    pub async fn get_pair_for_tokens(
        &self,
        token0: String,
        token1: String,
        config: &Config,
    ) -> String {
        let factory = FACTORY::new(config.factory.address, &self.client);

        factory
            .getPair(
                Address::from_str(&token0).unwrap(),
                Address::from_str(&token1).unwrap(),
            )
            .call()
            .await
            .unwrap()
            ._0
            .to_string()
    }
}
