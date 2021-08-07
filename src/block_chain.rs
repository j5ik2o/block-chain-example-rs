use crate::blocks::Blocks;
use crate::block::{Block, BlockId};

pub struct BlockChain(Blocks);

impl BlockChain {
  pub fn new() -> Self {
    Self(Blocks::new())
  }

  pub fn head_block(&self) -> &Block {
    self.0.head()
  }

  pub fn last_block(&self) -> &Block {
    self.0.last()
  }

  pub fn last_block_id(&self) -> &BlockId {
    &self.last_block().id
  }

  pub fn size(&self) -> usize {
    self.0.size()
  }

  pub fn append_new_block(&mut self, data: &[u8]) -> Block {
    let last_block = self.last_block().clone();
    let proof = last_block.proof.clone().next_proof();
    let new_block = Block::new(last_block, proof, data);
    self.0.push(new_block.clone());
    new_block
  }

  pub fn validate_blocks(blocks: &Blocks) -> bool {
    let mut result = true;
    let mut itr = blocks.0.iter();
    while let (Some(n1), Some(n2)) = (itr.next(), itr.next()) {
      result &= n2.validate(n1);
    }
    result
  }

  pub fn validate(&self) -> bool {
    Self::validate_blocks(&self.0)
  }

  pub fn resolve_conflicts<F>(&self, nodes: Vec<String>, get_full_chain: F) -> Option<BlockChain>
  where
    F: Fn(&String) -> Blocks,
  {
    nodes
      .iter()
      .map(|e| get_full_chain(e))
      .find(|e| e.size() > self.size() && Self::validate_blocks(e))
      .map(|e| BlockChain(e))
  }
}

#[cfg(test)]
mod tests {
  use crate::block_chain::BlockChain;

  #[test]
  fn test_block_chain() {
    let mut block_chain = BlockChain::new();

    block_chain.append_new_block("abc".as_bytes());
    block_chain.append_new_block("def".as_bytes());

    assert!(block_chain.validate())
  }
}
