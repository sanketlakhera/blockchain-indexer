// config.rs - handle env and config

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub eth_rpc_url: String,
    // add db url
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        // load variables from .env
        dotenvy::dotenv().ok();
        let eth_rpc_url =
            std::env::var("ETH_RPC_URL").context("ETH_RPC_URL environment variable")?;
        let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL environment variable")?;
        Ok(Self {
            eth_rpc_url,
            db_url,
        })
    }
}
