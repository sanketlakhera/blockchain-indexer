// use alloy::providers::{Provider, ProviderBuilder};
// use anyhow::Context;
use tracing::info;

// use alloy::consensus::Transaction;
// use alloy::eips::BlockNumberOrTag;
use alloy::consensus::Transaction;
use alloy::primitives::U256;
use backend::events::erc20::DecodedTransfer;

use backend::config::Config;
use backend::db;
use backend::fetcher::BlockFetcher;

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
    db::run_migration(&pool).await?;
    info!("Database migrations completed");
    let mut tx_pool = pool.begin().await?;

    // initialize block fetcher
    let fetcher = BlockFetcher::new(&config.eth_rpc_url)?;

    // fetch latest block
    let latest_block_num = fetcher.fetch_latest_block_number().await?;
    info!("Latest block number: {}", latest_block_num);

    let block = fetcher.fetch_latest_full_block().await?;
    info!("Latest block hash: {:?}", block.header.hash);

    let block_num = block.header.number as i64;
    let tx_count = block.transactions.len() as i32;
    db::insert_block(
        &mut *tx_pool,
        block_num,
        block.header.hash,
        block.header.parent_hash,
        block.header.timestamp as i64,
        tx_count,
    )
    .await?;

    // process transactions and events
    if let Some(txns) = block.transactions.as_transactions() {
        // info!("Total transactions in block: {}", txns.len());

        // for each transaction in the block,
        for (tx_idx, tx) in txns.iter().enumerate() {
            let tx_hash = *tx.inner.tx_hash();
            let from_addr = tx.inner.signer();
            let to_addr = tx.inner.to();
            let val = tx.inner.value();
            let gas_price = tx.inner.gas_price().map(U256::from);
            // let tx_idx = tx.inner.transaction_index();

            db::insert_transaction(
                &mut *tx_pool,
                tx_hash,
                block_num,
                from_addr,
                to_addr,
                val,
                gas_price,
                tx_idx as i32,
            )
            .await?;

            let receipt = fetcher.fetch_tx_receipt(tx_hash).await?;
            let logs = receipt.inner.logs();
            if !logs.is_empty() {
                info!("--- Smart Contract tx Hash: {:?} ---", tx_hash);
                for (idx, log) in logs.iter().enumerate() {
                    if let Some(transfer) = DecodedTransfer::from_log(log) {
                        info!(
                            "Log #{}: Decoded ERC20 Transfer - From: {:?}, To: {:?}, Value: {}",
                            idx, transfer.from, transfer.to, transfer.value
                        );
                        db::insert_erc20_transfer(
                            &mut *tx_pool,
                            tx_hash,
                            block_num,
                            log.address(),
                            transfer.from,
                            transfer.to,
                            transfer.value,
                            idx as i32,
                        )
                        .await?;
                    } else {
                        info!(
                            "Log #{}: Non-Transfer Log (Emitter={:?})",
                            idx,
                            log.address()
                        );
                    }
                }
                // break; // stop after inspecting the first transaction with logs
            }
        }
    }
    tx_pool.commit().await?;
    info!(
        "block {} and its transations persisted successfully!",
        block_num
    );

    Ok(())
}
