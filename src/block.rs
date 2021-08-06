use chrono::Utc;
use once_cell::sync::Lazy;

use crate::block_hash::BlockHash;
use crate::block_proof::BlockProof;
use crate::transaction::Transaction;
use crate::transactions::Transactions;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BlockId(u64);

pub const GENESIS_BLOCK_ID: BlockId = BlockId(0);
pub static GENESIS_BLOCK: Lazy<Block> =
    Lazy::new(|| Block::new_block(BlockProof::genesis_proof(), Transactions::empty(), None));

impl BlockId {
    pub fn is_valid(&self, prev_block_id: &Self) -> bool {
        self.0 == (prev_block_id.0 + 1)
    }

    pub fn to_hash(&self) -> BlockHash {
        BlockHash::new(self.0.to_string().as_str())
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Block {
    pub(crate) id: BlockId,
    previous_hash: Option<BlockHash>,
    timestamp: i64,
    pub(crate) proof: BlockProof,
    transactions: Transactions,
    pub(crate) hash: BlockHash,
}

impl Block {
    pub fn validate_hash(&self) -> bool {
        Self::generate_hash_from(self) == self.hash
    }

    pub fn validate_block(&self, prev_block: &Self) -> bool {
        self.validate_hash() && Self::_validate_block(self, prev_block)
    }

    fn generate_hash_from(block: &Block) -> BlockHash {
        Self::generate_hash(
            &block.id,
            &block.previous_hash,
            block.timestamp,
            &block.proof,
            &block.transactions,
        )
    }
    fn generate_hash(
        index: &BlockId,
        prav_hash_opt: &Option<BlockHash>,
        timestamp: i64,
        proof: &BlockProof,
        transactions: &Transactions,
    ) -> BlockHash {
        BlockHash::params(&vec![
            index.to_hash(),
            prav_hash_opt.clone().unwrap_or(BlockHash::new("")),
            BlockHash::new(timestamp.to_string().as_str()),
            proof.to_hash(),
            transactions.to_hash(),
        ])
    }
    fn new_block(
        proof: BlockProof,
        transactions: Transactions,
        last_block_opt: Option<Block>,
    ) -> Block {
        let now = Utc::now().timestamp_millis();
        last_block_opt
            .map(|lb| {
                let id = lb.id.next();
                Self {
                    id,
                    previous_hash: Some(lb.hash),
                    timestamp: now,
                    transactions: transactions.clone(),
                    proof: proof.clone(),
                    hash: Self::generate_hash(&GENESIS_BLOCK_ID, &None, now, &proof, &transactions),
                }
            })
            .unwrap_or(Self {
                id: GENESIS_BLOCK_ID,
                previous_hash: None,
                timestamp: now,
                transactions: transactions.clone(),
                proof: proof.clone(),
                hash: Self::generate_hash(&GENESIS_BLOCK_ID, &None, now, &proof, &transactions),
            })
    }

    fn _validate_block(block: &Block, prev_block: &Block) -> bool {
        match (prev_block, block) {
            (p, n) if !n.id.is_valid(&p.id) => false,
            (p, n) if !n.previous_hash.contains(&p.hash) => false,
            (p, n) if Self::generate_hash_from(&n) != n.hash => false,
            (p, n) if !n.proof.validate(&p.proof) => false,
            _ => true,
        }
    }

    pub fn new(last_block: Block, proof: BlockProof, transactions: Transactions) -> Self {
        Self::new_block(proof, transactions, Some(last_block))
    }
}

#[cfg(test)]
mod tests {
    use crate::block::{Block, GENESIS_BLOCK};
    use crate::block_proof::BlockProof;
    use crate::transactions::Transactions;

    #[test]
    fn test() {
        let a_block = Block::new(*GENESIS_BLOCK,
                                 GENESIS_BLOCK.proof.next_proof(),
                                 Transactions::empty());
        assert!(a_block.validate_block(&*GENESIS_BLOCK))
    }
}
