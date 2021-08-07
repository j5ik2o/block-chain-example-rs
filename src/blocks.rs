use crate::block::{Block, GENESIS_BLOCK};
use std::cmp::Ordering;
use crate::block_hash::BlockHash;
use std::slice::Iter;

#[derive(Debug)]
pub struct Blocks(pub(crate) Vec<Block>);

impl PartialEq for Blocks {
  fn eq(&self, other: &Self) -> bool {
    self.0.partial_cmp(&other.0) == Some(Ordering::Equal)
  }
}

impl Blocks {
  pub fn iter(&self) -> Iter<'_, Block> {
    self.0.iter()
  }

  pub fn new() -> Self {
    Self(vec![GENESIS_BLOCK.clone()])
  }

  pub fn size(&self) -> usize {
    self.0.len()
  }

  pub fn head(&self) -> &Block {
    self.0.first().unwrap()
  }

  pub fn last(&self) -> &Block {
    self.0.last().unwrap()
  }

  pub fn to_hash(&self) -> BlockHash {
    let vec = self
      .0
      .iter()
      .map(|v| v.to_hash())
      .collect::<Vec<BlockHash>>();
    BlockHash::params(&vec)
  }

  pub fn combine(&mut self, other: Self) {
    for e in other.0 {
      self.0.push(e);
    }
  }

  pub fn push(&mut self, other: Block) {
    self.0.push(other);
  }
}

#[cfg(test)]
mod tests {
  use crate::blocks::Blocks;
  use crate::block::GENESIS_BLOCK;

  #[test]
  fn test_push() {
    let h = GENESIS_BLOCK.clone();
    Blocks::new().push(h);
  }
}
