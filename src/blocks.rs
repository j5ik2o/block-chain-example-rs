use crate::block::Block;
use std::cmp::Ordering;
use crate::block_hash::BlockHash;
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct Blocks(Vec<Block>);

impl PartialEq for Blocks {
  fn eq(&self, other: &Self) -> bool {
    self.0.as_slice().partial_cmp(&other.0.as_slice()) == Some(Ordering::Equal)
  }
}

impl Blocks {
  pub fn iter(&self) -> Iter<'_, Block> {
    self.0.iter()
  }

  pub fn empty() -> Self {
    Self(vec![])
  }

  pub fn size(&self) -> usize {
    self.0.len()
  }

  pub fn head(&self) -> Block {
    self.0.first().unwrap().clone()
  }

  pub fn last(&self) -> Block {
    self.0.last().unwrap().clone()
  }

  pub fn to_hash(&self) -> BlockHash {
    let vec = self
      .0
      .iter()
      .map(|v| v.hash.clone())
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
