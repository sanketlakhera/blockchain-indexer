// use alloy::providers::{Provider, ProviderBuilder};
// use anyhow::Context;
use tracing::info;

// use alloy::consensus::Transaction;
// use alloy::eips::BlockNumberOrTag;
use backend::events::erc20::DecodedTransfer;

use backend::config::Config;
use backend::fetcher::BlockFetcher;
use backend::db;

// standard rust main fn cannot run async code directly
// we use #[tokio::main] to convert it into an async main fn
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //initialize log tracing
    tracing_subscriber::fmt::init();
    info!("Indexer application starting up...");

    // load config
    let config = Config::from_env()?;

    // init db
    let pool = db::init_pool(&config.db_url).await?;
    info!("Database connection established");

    // initialize block fetcher
    let fetcher = BlockFetcher::new(&config.eth_rpc_url)?;

    // fetch latest block
    let latest_block_num = fetcher.fetch_latest_block_number().await?;
    info!("Latest block number: {}", latest_block_num);

    let block = fetcher.fetch_latest_full_block().await?;
    info!("Latest block hash: {:?}", block.header.hash);

    // process transactions and events
    if let Some(txns) = block.transactions.as_transactions() {
        info!("Total transactions in block: {}", txns.len());

        // for each transaction in the block,
        for tx in txns {
            let tx_hash = tx.inner.tx_hash();
            let receipt = fetcher.fetch_tx_receipt(*tx_hash).await?;
            let logs = receipt.inner.logs();
            if !logs.is_empty() {
                info!("--- Smart Contract tx Hash: {:?} ---", tx_hash);
                for (idx, log) in logs.iter().enumerate() {
                    if let Some(transfer) = DecodedTransfer::from_log(log) {
                        info!(
                            "Log #{}: Decoded ERC20 Transfer - From: {:?}, To: {:?}, Value: {}",
                            idx, transfer.from, transfer.to, transfer.value
                        );
                    } else {
                        info!(
                            "Log #{}: Non-Transfer Log (Emitter={:?})",
                            idx,
                            log.address()
                        );
                    }
                }
                break; // stop after inspecting the first transaction with logs
            }
        }
    }

    Ok(())
}
