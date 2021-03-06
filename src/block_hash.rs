use sha2::{Sha256, Digest};

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
    hasher.update(self.0.clone());
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
    assert_eq!(
      r.to_hex_string(),
      "ba7816bf8f1cfea414140de5dae2223b0361a396177a9cb410ff61f2015ad"
    )
  }
}
