# Blockchain Indexer

A high-performance, modular Ethereum blockchain indexer written in Rust. Designed to ingest, decode, and persist blockchain blocks, transactions, and ERC-20 transfer event logs in real time with transactional integrity.

---

## 🏛️ System Architecture

```mermaid
flowchart TD
    RPC["Ethereum JSON-RPC Node\n(HTTP / WebSocket)"]
    
    subgraph Ingestion["Ingestion Engine (Rust + Tokio)"]
        Fetcher["Block & Receipt Fetcher\n(Alloy Provider)"]
        Decoder["Event Log Decoder\n(alloy::sol! compile-time ABI)"]
        Mapper["Type Mapper\n(Alloy B256/U256 to SQL Types)"]
    end
    
    subgraph Persistence["Storage Layer (PostgreSQL)"]
        Tx["Atomic Transaction\n(sqlx::Transaction)"]
        T_Blocks[("blocks")]
        T_Tx[("transactions")]
        T_Logs[("erc20_transfers")]
        T_Checkpoints[("checkpoints")]
    end

    RPC -->|Block Headers & Receipts| Fetcher
    Fetcher -->|Raw Event Logs| Decoder
    Decoder -->|Structured Transfers| Mapper
    Mapper -->|Atomic Batch Write| Tx
    
    Tx --> T_Blocks
    Tx --> T_Tx
    Tx --> T_Logs
    Tx --> T_Checkpoints
```

---

## ✨ Features

- **Asynchronous Ingestion Pipeline:** Built on the `tokio` async runtime for non-blocking network I/O and concurrent block streaming.
- **Type-Safe Ethereum Client:** Integrates Paradigm's `alloy` suite for modern, ergonomic interaction with Ethereum execution layer JSON-RPC endpoints.
- **Compile-Time ABI Decoding:** Leverages the `alloy::sol!` macro to generate type-safe bindings for Solidity event ABIs (ERC-20 `Transfer` events decoded with zero runtime overhead).
- **Atomic Transactional Persistence:** Implements strict ACID persistence using PostgreSQL and `sqlx`. Every block and its associated transactions and decoded transfer logs are persisted within a single database transaction, preventing partial ingestion or inconsistent states during failures.
- **Automated Schema Migrations:** Self-healing startup routine using embedded migrations (`sqlx::migrate!`).
- **Structured Observability:** Complete tracing and contextual logging powered by the `tracing` ecosystem.

---

## 🗄️ Database Schema

The database schema models blockchain execution state cleanly across relational tables:

| Table | Primary Key | Key Columns Indexed | Description |
| :--- | :--- | :--- | :--- |
| `blocks` | `block_number` | `block_hash`, `parent_hash`, `timestamp` | Block headers, gas usage, validator details |
| `transactions` | `transaction_hash` | `block_number`, `from_address`, `to_address` | Transaction execution status, gas used, values |
| `erc20_transfers`| `id` (bigserial) | `contract_address`, `from_address`, `to_address` | Decoded token transfer logs and values |
| `checkpoints` | `id` | `last_synced_block`, `updated_at` | Stateful resumption marker for daemon restarts |

---

## 🚀 Quickstart

### Prerequisites
- **Rust:** 1.75+ (`rustup default stable`)
- **PostgreSQL:** 14+ (Local instance or Docker)
- **Ethereum RPC URL:** Infura, Alchemy, or a local execution client (Geth/Reth)

### Setup & Run

1. **Clone the repository:**
   ```bash
   git clone https://github.com/sanketlakhera/blockchain-indexer.git
   cd blockchain-indexer/backend
   ```

2. **Configure Environment:**
   Create `.env` inside `backend/`:
   ```bash
   DATABASE_URL="postgres://postgres:postgres@localhost:5432/indexer_db"
   ETH_RPC_URL="https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY"
   RUST_LOG="info,indexer=debug"
   ```

3. **Run Migrations & Ingest:**
   ```bash
   cargo run
   ```

---

## 🗺️ Roadmap & Milestones

- [x] **Milestone 1: RPC Client & Block Ingestion** (Alloy provider integration, block & receipt fetching).
- [x] **Milestone 2: Type-Safe Event Decoding** (`alloy::sol!` compilation for ERC-20 transfers).
- [x] **Milestone 3: Transactional Persistence Layer** (SQLx type mapping, atomic PostgreSQL transaction boundary).
- [ ] **Milestone 4: Continuous Daemon & Checkpointing** (Stateful checkpoint tracking and live chain tip polling).
- [ ] **Milestone 5: Reorg Detection & Rollback Engine** (Parent hash validation and transactional state rollbacks).
- [ ] **Milestone 6: High-Throughput Buffering** (Bounded channel worker pools with backpressure).
- [ ] **Milestone 7: Query API** (Axum REST API with indexed pagination and filtering).

---

## 📜 License
MIT License.