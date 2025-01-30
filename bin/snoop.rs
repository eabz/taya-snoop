use alloy::sol_types::SolEvent;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use taya_snoop::{
    configs::Config,
    db::Database,
    handlers::{
        burn::{handle_burns, Burn},
        mint::{handle_mint, Mint},
        pairs::handle_pairs,
        swap::{handle_swaps, Swap},
        sync::{handle_syncs, Sync},
        transfer::{handle_transfer, Transfer},
    },
    rpc::Rpc,
};

#[tokio::main()]
async fn main() {
    let log = SimpleLogger::new().with_level(LevelFilter::Info);

    let config = Config::new();

    if config.debug {
        log.with_level(LevelFilter::Debug).init().unwrap();
    } else {
        log.init().unwrap();
    }

    let rpc = Rpc::new(&config).await;

    let db =
        Database::new(config.db_url.clone(), config.chain.clone()).await;

    info!("Starting Taya Snoop.");

    loop {
        sync_chain(&rpc, &db, &config).await;
    }
}

async fn sync_chain(rpc: &Rpc, db: &Database, config: &Config) {
    let mut last_synced_block = db.get_last_block_indexed().await;

    if last_synced_block < config.factory.start_block {
        last_synced_block = config.factory.start_block
    }

    let last_chain_block = rpc.get_last_block().await as i64;

    let sync_blocks: Vec<i64> =
        (last_synced_block + 1..=last_chain_block).collect();

    let sync_blocks_chunks: std::slice::Chunks<'_, i64> =
        sync_blocks.chunks(config.batch_size);

    info!(
        "Start sync from block {} to {} with {} blocks each batch",
        last_synced_block, last_chain_block, config.batch_size
    );

    for block_chunk in sync_blocks_chunks {
        let first_block = block_chunk[0];
        let last_block = block_chunk[block_chunk.len() - 1];

        let pair_logs = rpc
            .get_factory_logs_batch(
                first_block as u64,
                last_block as u64,
                config,
            )
            .await;

        handle_pairs(pair_logs, db, rpc).await;

        let pairs = db.get_pairs().await;

        if !pairs.is_empty() {
            let mut event_logs = rpc
                .get_pairs_logs_batch(
                    &pairs,
                    first_block as u64,
                    last_block as u64,
                )
                .await;

            let mut count_mints = 0;
            let mut count_burns = 0;
            let mut count_swaps = 0;
            let mut count_syncs = 0;
            let mut count_transfers = 0;

            event_logs.sort_unstable_by_key(|log| {
                let block_num = log.block_number.unwrap_or_default();
                let log_index = log.log_index.unwrap_or_default();
                (block_num, log_index)
            });

            for log in event_logs {
                match log.topic0() {
                    Some(topic_raw) => {
                        let topic = topic_raw.to_string();

                        if topic == Mint::SIGNATURE_HASH.to_string() {
                            handle_mint(log, db).await;
                            count_mints += 1;
                        } else if topic == Burn::SIGNATURE_HASH.to_string()
                        {
                            handle_burns(log, db).await;
                            count_burns += 1;
                        } else if topic == Swap::SIGNATURE_HASH.to_string()
                        {
                            handle_swaps(log, db).await;
                            count_swaps += 1;
                        } else if topic == Sync::SIGNATURE_HASH.to_string()
                        {
                            handle_syncs(log, db, rpc, config).await;
                            count_syncs += 1;
                        } else if topic
                            == Transfer::SIGNATURE_HASH.to_string()
                        {
                            handle_transfer(log, db).await;
                            count_transfers += 1;
                        }
                    }
                    None => continue,
                }
            }

            info!("Procesed {} mints {} burns {} swaps {} sync and {} transfer events", count_mints, count_burns, count_swaps, count_syncs, count_transfers);
        }

        db.update_last_block_indexed(last_block).await;
    }
}
