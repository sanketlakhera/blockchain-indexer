# Blockchain Indexer

## Backend
Learned so far
- how to initialize tokio async
- how to fetch blocks using alloy provider
- get transaction receipts and execution status
- alloy macro `sol!` for strongly typed decoding of ERC20 transfer events from log topics and data


### ERC20 transfer event
Think bank transactions. when we transfer money from "A" to "B", we actually do not transfer money. we just update the balances of A and B in database. Etherum is like one giant database. Instead of bank it is owned by thousands of computers. Ethereum does not know about currency (like USDT, SHIBA etc.).

So ERC20 standard specifies an "Transfer" event. Whenever A sends token to B, contract emits Transfer event. Everyone agrees on this standard.

Balances are stored in smart contracts.

Just like our A -> B transfer, in ethereum when "A" sends token to "B", the contract executes and updates the balances.