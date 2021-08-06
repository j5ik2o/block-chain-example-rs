use crate::blocks::Blocks;
use crate::block::{Block, BlockId};
use crate::transactions::Transactions;

pub struct BlockChain(Blocks);

impl BlockChain {
  pub fn head_block(&self) -> Block {
    self.0.head()
  }

  pub fn last_block(&self) -> Block {
    self.0.last()
  }

  pub fn last_block_id(&self) -> BlockId {
    self.last_block().id
  }

  pub fn size(&self) -> usize {
    self.0.size()
  }

  pub fn new_block(&mut self, transactions: Transactions) -> Block {
    let block = Block::new(
      self.last_block(),
      self.last_block().proof.next_proof(),
      transactions,
    );
    self.0.push(block.clone());
    block
  }

  pub fn validate_blocks(blocks: &Blocks) -> bool {
    let mut i = blocks.iter();
    let mut result = true;
    while let (Some(n1), Some(n2)) = (i.next(), i.next()) {
      result &= n1.validate_block(n2)
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
