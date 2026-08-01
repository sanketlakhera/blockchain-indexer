use alloy::eips::BlockNumberOrTag;
use alloy::primitives::B256;
use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use alloy::rpc::types::{Block, TransactionReceipt};
use alloy::transport::http::{Client, Http};
use anyhow::Context;

type HttpProvider = RootProvider<Http<Client>>;

pub struct BlockFetcher {
    provider: HttpProvider,
}

impl BlockFetcher {
    // instantiates a new blockfetcher using the given rpc url
    pub fn new(rpc_url_str: &str) -> anyhow::Result<Self> {
        let rpc_url = rpc_url_str.parse()?;
        let provider = ProviderBuilder::new().connect_http(rpc_url);
        Ok(Self { provider })   
    }

    // fetches the latest block number on the chain
    pub fn fetch_latest_block_number(&self) -> anyhow::Result<u64> {
        // we will call provider inside an async method
        // methods calling .await must be async fn
        todo!()   
    }

    // fetches a block with full transactions
    pub fn fetch_latest_full_block(&self) -> anyhow::Result<Block> {
        todo!()
    }

    // fetch transaction receipt by hash
    pub fn fetch_tx_receipt(&self, hash: B256) -> anyhow::Result<TransactionReceipt> {
        todo!()
    }
}