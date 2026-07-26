use alloy::providers::{Provider, ProviderBuilder};
use anyhow::Context;
use tracing::info;

use alloy::consensus::Transaction;
use alloy::eips::BlockNumberOrTag;
use alloy::rpc::types::BlockTransactionsKind;
// standard rust main fn cannot run async code directly
// we use #[tokio::main] to convert it into an async main fn
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load variables from .env
    dotenvy::dotenv().ok();

    //initialize log tracing
    tracing_subscriber::fmt::init();
    info!("Indexer application starting up...");

    // get url from env var
    let rpc_url_str = std::env::var("ETH_RPC_URL").context("ETH_RPC_URL environment variable")?;
    let rpc_url = rpc_url_str.parse()?;
    // info!("Using RPC URL: {}", rpc_url);

    // build http provider
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // get latest block number
    let latest_block = provider.get_block_number().await?;
    info!("Latest block number: {}", latest_block);

    // fetch full block details including full transaction objects
    let block = provider
        .get_block_by_number(BlockNumberOrTag::Latest)
        .full() // in alloy we need to chain .full() to get full transaction objects, else it returns only the hashes
        .await?
        .context("Block not found")?;
    let header = &block.header;
    info!("----------------------------------------");
    info!("Block Hash: {:?}", header.hash);
    info!("Parent Hash: {:?}", header.parent_hash);
    info!("Timestamp: {}", header.timestamp);
    info!("Gas Used: {}", header.gas_used);
    info!("----------------------------------------");

    // inspect transaction list
    if let Some(txn) = block.transactions.as_transactions() {
        info!("Total transactions in Block: {}", txn.len());

        // for (idx, tx) in txn.iter().take(5).enumerate() {
        //     info!(
        //         "Tx #{}: Hash={:?}, From={:?}, To={:?}, Value={} wei",
        //         idx,
        //         tx.inner.tx_hash(),
        //         tx.inner.signer(),
        //         tx.inner.to(),
        //         tx.inner.value()
        //     );
        // }

        if let Some(first_tx) = txn.first() {
            let tx_hash = first_tx.inner.tx_hash();
            info!("Fetching receipt for tx Hash: {:?}", tx_hash);
            if let Some(receipt) = provider.get_transaction_receipt(*tx_hash).await? {
                info!("Txn");
                info!("Execution Status: {:?}", receipt.status());
                info!("Gas used: {}", receipt.gas_used);
                info!("Total Logs Emitted: {}", receipt.inner.logs().len());

                for (idx, log) in receipt.inner.logs().iter().enumerate() {
                    info!("Log #{}: Emitter Address={:?}, Topics={}", idx, log.address(), log.topics().len());
                }
            }
        }
    }
    Ok(())
}
