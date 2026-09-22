# Blockchain Indexer - Progress & Resume Notes

## 📌 Current Status: Week 3 (Persistent Storage Layer)

### ✅ Completed Milestones
1. **PostgreSQL Schema & Migrations**:
   - Migration file: [`backend/migrations/20260802114536_create_indexer_tables.sql`](file:///home/madara/Documents/projects/rust/indexer/backend/migrations/20260802114536_create_indexer_tables.sql)
   - Created tables: `blocks`, `transactions`, `erc20_transfers`, and `_sqlx_migrations`.
   - Auto-migrates on boot using `sqlx::migrate!("./migrations")` in [`backend/src/db.rs`](file:///home/madara/Documents/projects/rust/indexer/backend/src/db.rs).
2. **Type Mapping & Database Insertion Helpers**:
   - Mapped Alloy types (`B256`, `Address`, `U256`) to PostgreSQL types (`VARCHAR(66)`, `VARCHAR(42)`, `NUMERIC`).
   - Insertion functions accept `&mut PgConnection` for transactional composition.
3. **Concept 3: Atomic Block Persistence**:
   - All writes for a block (header, all transactions, and all decoded ERC20 transfer events) run atomically inside a single `sqlx::Transaction` (`db_tx`).
   - Verified live indexing in PostgreSQL: **2 blocks, 277 transactions, and 149 ERC20 transfers** persisted cleanly!

---

## 🚀 Up Next: Concept 4 — Checkpointing & Resume Logic

### 1. Problem Statement
A blockchain indexer is a long-running daemon. In production, indexer instances restart during deployments, configuration changes, or system maintenance.

Without persistent state tracking:
- If the indexer restarts and queries `latest_block`, it creates a **permanent data gap** (missing all blocks minted while it was offline).
- If the indexer restarts with no state, it has to re-index from genesis, wasting days of compute and millions of RPC requests.

### 2. Why & How Solution Exists
We implement a **Stateful Checkpointing Architecture**:
1. **Checkpoints Table**: A dedicated table stores `(id, last_synced_block, updated_at)`.
2. **Atomic Ingestion Step**: Inside the same `db_tx` that saves the block and transactions, we update the checkpoint:
   ```sql
   INSERT INTO checkpoints (id, last_synced_block, updated_at)
   VALUES ($1, $2, CURRENT_TIMESTAMP)
   ON CONFLICT (id) 
   DO UPDATE SET 
       last_synced_block = EXCLUDED.last_synced_block,
       updated_at = CURRENT_TIMESTAMP;
   ```
   Because it is inside the same transaction, the checkpoint advances **if and only if** all block data is successfully committed to disk.
3. **Continuous Ingestion Loop**:
   - On startup: Query `SELECT last_synced_block FROM checkpoints WHERE id = 'ethereum_mainnet'`.
   - If present: $\text{start\_block} = \text{last\_synced\_block} + 1$.
   - If empty: Start from a configured start block or `latest_block`.
   - Process blocks in a loop up to `latest_block`. When caught up with the tip of the chain, wait/sleep for the next block.

---

### 3. Step-by-Step Action Items

#### Step 1: Create Checkpoints Table Migration
Create a new migration file: [`backend/migrations/20260816000000_create_checkpoints_table.sql`](file:///home/madara/Documents/projects/rust/indexer/backend/migrations/)

```sql
CREATE TABLE IF NOT EXISTS checkpoints (
    id VARCHAR(50) PRIMARY KEY,
    last_synced_block BIGINT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

#### Step 2: Add Checkpoint Methods in [`backend/src/db.rs`](file:///home/madara/Documents/projects/rust/indexer/backend/src/db.rs)
Add helper functions to fetch and update checkpoints:

```rust
// Fetch last synced block from checkpoints table
pub async fn get_last_synced_block(pool: &PgPool, id: &str) -> Result<Option<i64>> {
    let row: Option<(i64,)> = sqlx::query_as(
        r#"
        SELECT last_synced_block FROM checkpoints WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0))
}

// Update checkpoint atomically within the block's transaction
pub async fn update_checkpoint(
    conn: &mut PgConnection,
    id: &str,
    block_number: i64,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO checkpoints (id, last_synced_block, updated_at)
        VALUES ($1, $2, CURRENT_TIMESTAMP)
        ON CONFLICT (id) 
        DO UPDATE SET 
            last_synced_block = EXCLUDED.last_synced_block,
            updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(id)
    .bind(block_number)
    .execute(conn)
    .await?;

    Ok(())
}
```

#### Step 3: Add Block-by-Number Fetching in [`backend/src/fetcher.rs`](file:///home/madara/Documents/projects/rust/indexer/backend/src/fetcher.rs)
Add a method to fetch a full block by a specific block number:

```rust
pub async fn fetch_block_by_number(&self, block_number: u64) -> anyhow::Result<Block> {
    let block = self
        .provider
        .get_block_by_number(BlockNumberOrTag::Number(block_number))
        .full()
        .await?
        .context(format!("Block {} not found", block_number))?;

    Ok(block)
}
```

#### Step 4: Build the Ingestion Loop in [`backend/src/main.rs`](file:///home/madara/Documents/projects/rust/indexer/backend/src/main.rs)
Structure the main loop:
1. Determine `current_block` from checkpoint.
2. In a `loop { ... }`:
   - Fetch `latest_block_num`.
   - If `current_block <= latest_block_num`:
     - Fetch block with `fetcher.fetch_block_by_number(current_block as u64)`.
     - Open `db_tx`, persist block, transactions, ERC20 transfers.
     - `db::update_checkpoint(&mut *db_tx, checkpoint_id, current_block).await?`.
     - `db_tx.commit().await?`.
     - `current_block += 1`.
   - Else: Sleep for ~12 seconds (`tokio::time::sleep`) before checking again.

---

## 🎯 What Comes After Concept 4
1. **Automated Unit & Integration Tests**: Testing database storage and decoding logic with test fixtures.
2. **Week 4 (High-Throughput Architecture)**:
   - Ingestion queues (`tokio::sync::mpsc`)
   - Parallel worker pools for receipt fetching
   - Multi-row batch inserts (`UNNEST` / batch queries) for $10\times$ write throughput.
