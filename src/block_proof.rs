use crate::block_hash::BlockHash;

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct BlockProof(u64);

pub const DIFFICULTY: usize = 1;

impl BlockProof {
  pub fn to_hash(&self) -> BlockHash {
    BlockHash::new(self.0.to_string().as_str())
  }

  pub fn next_proof(self) -> BlockProof {
    Self::proof_of_work(self)
  }

  pub fn validate(&self, prev_proof: &BlockProof) -> bool {
    Self::validate_proof(prev_proof, self)
  }

  fn proof_of_work(last_proof: BlockProof) -> BlockProof {
    let result = itertools::iterate(last_proof, |p| p.clone())
      .enumerate()
      .into_iter()
      .filter(|(index, p)| Self::validate_proof(p, &BlockProof(*index as u64)))
      .map(|v| BlockProof(v.0 as u64))
      .next()
      .unwrap();
    result
  }

  fn validate_proof(last_proof: &BlockProof, proof: &BlockProof) -> bool {
    let v = vec![last_proof.to_hash(), proof.to_hash()];
    let hash = BlockHash::params(&v).to_hex_string();
    let result = hash.starts_with(&"0".repeat(DIFFICULTY));
    result
  }

  pub fn genesis_proof() -> BlockProof {
    Self::proof_of_work(BlockProof(0))
  }
}

#[cfg(test)]
mod tests {
  use crate::block_proof::BlockProof;

  #[test]
  fn test_proof() {
    let genesis_proof = BlockProof::genesis_proof();
    let next = genesis_proof.clone().next_proof();
    assert!(next.validate(&genesis_proof))
  }
}
