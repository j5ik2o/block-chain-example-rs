use crate::transaction::Transaction;
use crate::block_hash::BlockHash;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Transactions(Vec<Transaction>);

impl PartialEq for Transactions {
  fn eq(&self, other: &Self) -> bool {
    self.0.as_slice().partial_cmp(&other.0.as_slice()) == Some(Ordering::Equal)
  }
}

impl PartialOrd for Transactions {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.as_slice().partial_cmp(&other.0.as_slice())
  }
}

impl Transactions {
  pub fn empty() -> Self {
    Self(vec![])
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

  pub fn push(&mut self, other: Transaction) {
    self.0.push(other);
  }
}
