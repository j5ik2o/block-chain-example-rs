# block-chain-example-rs

```rust
let mut block_chain = BlockChain::new();

block_chain.append_new_block("abc".as_bytes());
block_chain.append_new_block("def".as_bytes());

assert!(block_chain.validate())
```