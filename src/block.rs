use crate::block_hash::BlockHash;
use crate::block_proof::BlockProof;
use crate::transcation::Transaction;
use chrono::Utc;
use crate::transactions::Transactions;

pub struct BlockId(u64);
pub const GENESIS_BLOCK_ID: BlockId = BlockId(0);

impl BlockId {
    pub fn is_valid(&self, prev_block_id: Self) -> bool {
        self.0 == (prev_block_id.0 + 1)
    }

    pub fn to_hash(&self) -> BlockHash {
        BlockHash::new(self.0.to_string().as_str())
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

pub struct Block {
    id: BlockId,
    previous_hash: Option<BlockHash>,
    timestamp: i64,
    proof: BlockProof,
    transactions: Transactions,
    hash: BlockHash,
}

impl Block {
    fn generate_hash(index: BlockId, prav_hash_opt: Option<BlockHash>, timestamp: i64, proof: BlockProof, transactions: Transactions) -> BlockHash {
        BlockHash::params(&vec![index.to_hash(),
                                prav_hash_opt.unwrap_or(BlockHash::new("")),
                                BlockHash::new(timestamp.to_string().as_str()),
            proof.to_hash(),
            transactions.to_hash()
        ])
    }
    fn new_block(proof: BlockProof, transactions: Transactions, last_block_opt: Option<Block>) -> Block {
        let now = Utc::now().timestamp_millis();
        last_block_opt.map(|lb| {
            let id = lb.id.next();
            Self {
                id,
                previous_hash: Some(lb.hash),
                timestamp: now,
                transactions: transactions.clone(),
                proof: proof.clone(),
                hash: Self::generate_hash(GENESIS_BLOCK_ID, None, now, proof, transactions)
            }
        }).unwrap_or(Self {
            id: GENESIS_BLOCK_ID,
            previous_hash: None,
            timestamp: now,
            transactions: transactions.clone(),
            proof: proof.clone(),
            hash: Self::generate_hash(GENESIS_BLOCK_ID, None, now, proof, transactions)
        })
    }
    // pub fn new(last_block: Block, proof: BlockProof) -> Self {
    //     Self {
    //         id
    //     }
    // }
}



