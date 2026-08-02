-- 1. Blocks Table
CREATE TABLE IF NOT EXISTS blocks (
    number BIGINT PRIMARY KEY,
    hash VARCHAR(66) NOT NULL UNIQUE,
    parent_hash VARCHAR(66) NOT NULL,
    timestamp BIGINT NOT NULL,
    transactions_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

-- 2. Transactions Table
CREATE TABLE IF NOT EXISTS transactions (
    hash VARCHAR(66) PRIMARY KEY,
    block_number BIGINT NOT NULL REFERENCES blocks(number) ON DELETE CASCADE,
    from_address VARCHAR(42) NOT NULL,
    to_address VARCHAR(42),
    value NUMERIC NOT NULL,
    gas_price NUMERIC,
    transaction_index INT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Index for fetching transactions in block
CREATE INDEX IF NOT EXISTS idx_transactions_block_number ON transactions(block_number);
CREATE INDEX IF NOT EXISTS idx_transactions_from_address ON transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_transactions_to_address ON transactions(to_address);

-- erc20 events
CREATE TABLE IF NOT EXISTS erc20_transfers(
    id BIGSERIAL PRIMARY KEY,
    transaction_hash VARCHAR(66) NOT NULL REFERENCES transactions(hash) ON DELETE CASCADE,
    block_number BIGINT NOT NULL REFERENCES blocks(number) ON DELETE CASCADE,
    contract_address VARCHAR(42) NOT NULL,
    from_address VARCHAR(42) NOT NULL,
    to_address VARCHAR(42) NOT NULL,
    amount NUMERIC NOT NULL,
    log_index INT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_transfer_per_log UNIQUE (transaction_hash, log_index)
);

-- Indexes for erc20 transfer queries
CREATE INDEX IF NOT EXISTS idx_erc20_transfers_contract ON erc20_transfers(contract_address);
CREATE INDEX IF NOT EXISTS idx_erc20_transfers_from ON erc20_transfers(from_address);
CREATE INDEX IF NOT EXISTS idx_erc20_transfers_to ON erc20_transfers(to_address);
CREATE INDEX IF NOT EXISTS idx_erc20_transfers_block ON erc20_transfers(block_number);