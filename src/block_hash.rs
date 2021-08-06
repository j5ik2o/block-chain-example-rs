use sha2::{Sha256, Digest};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BlockHash(String);

impl BlockHash {
  pub fn new(value: &str) -> Self {
    Self(String::from(value))
  }
  pub fn params(values: &[BlockHash]) -> Self {
    let r = values
      .iter()
      .fold("".to_owned(), |s, b| s + ":" + b.to_hex_string().as_str());
    Self::new(&r)
  }

  pub fn to_hex_string(&self) -> String {
    let mut hasher = Sha256::new();
    hasher.update(self.clone().0);
    let hash = hasher.finalize();
    hash[..]
      .iter()
      .fold("".to_owned(), |s, b| s + &format!("{:x}", b))
  }
}

#[cfg(test)]
mod tests {
  use crate::block_hash::BlockHash;

  #[test]
  fn test_hash() {
    let r = BlockHash::new("abc");
    println!("hash = {:?}", r.to_hex_string());
  }
}
