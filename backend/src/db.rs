use alloy::primitives::{Address, B256, U256};
use anyhow::Result;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn init_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn run_migration(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}

// insert block idempotent with on conflict do nothing
pub async fn insert_block(
    pool: &PgPool,
    number: i64,
    hash: B256,
    parent_hash: B256,
    timestamp: i64,
    tx_count: i32,
) -> Result<()> {
    sqlx::query(
        r#"
    INSERT INTO blocks (number, hash, parent_hash, timestamp, transaction_count)
    VALUES ($1, $2, $3, $4, $5)
    ON CONFLICT (number) DO NOTHING"#,
    )
    .bind(number)
    .bind(format!("{:#x}", hash))
    .bind(format!("{:#x}", parent_hash))
    .bind(timestamp)
    .bind(tx_count)
    .execute(pool)
    .await?;

    Ok(())
}

// insert transaction
pub async fn insert_transaction(
    pool: &PgPool,
    tx_hash: B256,
    block_number: i64,
    from: Address,
    to: Option<Address>,
    value: U256,
    gas_price: Option<U256>,
    tx_index: i32,
) -> Result<()> {
    let to_str = to.map(|addr| format!("{:#x}", addr));
    let gas_price_str = gas_price.map(|gp| gp.to_string());

    sqlx::query(
        r#"
        INSERT INTO transactions (hash, block_number, from_address, to_address, value, gas_price, transaction_index)
        VALUES ($1, $2, $3, $4, $5::numeric, $6::numeric, $7)
        ON CONFLICT (hash) DO NOTHING 
        "#,
    )
    .bind(format!("{:#x}", tx_hash))
    .bind(block_number)
    .bind(format!("{:#x}", from))
    .bind(to_str)
    .bind(value.to_string())
    .bind(gas_price_str)
    .bind(tx_index)
    .execute(pool)
    .await?;

    Ok(())
}

// insert ERC20 Transfer
pub async fn insert_erc20_transfer(
    pool: &PgPool,
    tx_hash: B256,
    block_number: i64,
    contract_address: Address,
    from: Address,
    to: Address,
    amount: U256,
    log_index: i32,
) -> Result<()> {
    sqlx::query(
    r#"
    INSERT INTO erc20_transfers (transactions_hash, block_number, contract_address, from_address, to_address, amount, log_index)
    VALUES ($1, $2, $3, $4, $5, $6::numeric, $7)
    ON CONFLICT (transaction_hash, log_index) DO NOTHING
    "#,)
    .bind(format!("{:#x}", tx_hash))
    .bind(block_number)
    .bind(format!("{:#x}", contract_address))
    .bind(format!("{:#x}", from))
    .bind(format!("{:#x}", to))
    .bind(amount.to_string())
    .bind(log_index)
    .execute(pool)
    .await?;

    Ok(())
}
