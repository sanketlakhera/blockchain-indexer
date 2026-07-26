use tracing::info;
// standard rust main fn cannot run async code directly
// we use #[tokio::main] to convert it into an async main fn
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load variables from .env
    dotenvy::dotenv().ok();

    //initialize log tracing
    tracing_subscriber::fmt::init();
    info!("Indexer application starting up...");

    Ok(())
}
