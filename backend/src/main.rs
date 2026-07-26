use tracing::info;
use alloy::providers::{Provider, ProviderBuilder};
use anyhow::Context;
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

    Ok(())
}
