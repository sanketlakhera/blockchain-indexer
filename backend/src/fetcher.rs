use alloy::eips::BlockNumberOrTag;
use alloy::primitives::B256;
use alloy::providers::{Provider, ProviderBuilder, DynProvider} ;
use alloy::rpc::types::{Block, TransactionReceipt};
// use alloy::transports::http;
use anyhow::Context;

type HttpProvider = DynProvider;

pub struct BlockFetcher {
    provider: HttpProvider,
}

impl BlockFetcher {
    // instantiates a new blockfetcher using the given rpc url
    pub fn new(rpc_url_str: &str) -> anyhow::Result<Self> {
        let rpc_url = rpc_url_str.parse()?;
        let provider = ProviderBuilder::new().connect_http(rpc_url);
        Ok(Self { provider: DynProvider::new(provider) })   
    }

    // fetches the latest block number on the chain
    pub async fn fetch_latest_block_number(&self) -> anyhow::Result<u64> {
        // we will call provider inside an async method
        // methods calling .await must be async fn
        let block_num = self.provider.get_block_number().await?;
        Ok(block_num)
    }

    // fetches a block with full transactions
    pub async fn fetch_latest_full_block(&self) -> anyhow::Result<Block> {
        let block = self.provider.get_block_by_number(BlockNumberOrTag::Latest).full().await?.context("Latest block not found")?;
        Ok(block)
    }

    // fetch transaction receipt by hash
    pub async fn fetch_tx_receipt(&self, hash: B256) -> anyhow::Result<TransactionReceipt> {
        let receipt = self.provider.get_transaction_receipt(hash).await?.context("transaction receipt not found")?;
        Ok(receipt)
    }
}