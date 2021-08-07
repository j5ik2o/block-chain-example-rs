use chrono::Utc;
use once_cell::sync::Lazy;

use crate::block_hash::BlockHash;
use crate::block_proof::BlockProof;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BlockId(u64);

pub const GENESIS_BLOCK_ID: BlockId = BlockId(0);
pub static GENESIS_BLOCK: Lazy<Block> =
  Lazy::new(|| Block::new_block(BlockProof::genesis_proof(), Vec::<u8>::new(), None));

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
  data: Vec<u8>,
}

impl Block {

  pub fn as_data(&self) -> &Vec<u8> {
    &self.data
  }

  pub fn to_hash(&self) -> BlockHash {
    Self::generate_hash(
      &self.id,
      &self.previous_hash,
      self.timestamp,
      &self.proof,
    )
  }

  pub fn validate_block(&self, prev_block: &Block) -> bool {
    match (prev_block, self) {
      (p, n) if !n.id.is_valid(&p.id) => false,
      (p, n) if !n.previous_hash.contains(&p.to_hash()) => false,
      (_, n) if Self::generate_hash_from(&n) != n.to_hash() => false,
      (p, n) if !n.proof.validate(&p.proof) => false,
      _ => true,
    }
  }

  pub fn validate_hash(&self) -> bool {
    let gen_hash = Self::generate_hash_from(self);
    gen_hash == self.to_hash()
  }

  pub fn validate(&self, prev_block: &Self) -> bool {
    let b_hash = self.validate_hash();
    let b_block = self.validate_block(prev_block);
    b_hash && b_block
  }

  fn generate_hash_from(block: &Block) -> BlockHash {
    Self::generate_hash(
      &block.id,
      &block.previous_hash,
      block.timestamp,
      &block.proof,
    )
  }

  fn generate_hash(
    index: &BlockId,
    prev_hash_opt: &Option<BlockHash>,
    timestamp: i64,
    proof: &BlockProof,
  ) -> BlockHash {
    let v = [
      index.to_hash(),
      prev_hash_opt.clone().unwrap_or(BlockHash::new("")),
      BlockHash::new(timestamp.to_string().as_str()),
      proof.to_hash(),
    ];
    BlockHash::params(&v)
  }

  fn new_block(
    proof: BlockProof,
    data: Vec<u8>,
    last_block_opt: Option<Block>,
  ) -> Block {
    let now = Utc::now().timestamp_millis();
    match last_block_opt {
      None => Self {
        id: GENESIS_BLOCK_ID,
        previous_hash: None,
        timestamp: now,
        data,
        proof,
      },
      Some(lb) => {
        Self {
          id: lb.id.next(),
          previous_hash: Some(lb.to_hash()),
          timestamp: now,
          data,
          proof,
        }
      }
    }
  }

  pub fn new(last_block: Block, proof: BlockProof, data: Vec<u8>) -> Self {
    Self::new_block(proof, data, Some(last_block))
  }
}

#[cfg(test)]
mod tests {
  use crate::block::{Block, GENESIS_BLOCK};

  #[test]
  fn test() {
    let prev_block = GENESIS_BLOCK.clone();
    let a_block = Block::new(
      prev_block.clone(),
      prev_block.clone().proof.next_proof(),
      Vec::<u8>::new(),
    );
    assert!(a_block.validate(&prev_block))
  }
}
