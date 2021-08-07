# block-chain-example-rs

```rust
let mut block_chain = BlockChain::new();

let block1 = block_chain.append_new_block("abc".as_bytes());
println("block1 = {:?}", block1);

let block2 = block_chain.append_new_block("def".as_bytes());
println("block2 = {:?}", block2);

assert!(block_chain.validate())
```