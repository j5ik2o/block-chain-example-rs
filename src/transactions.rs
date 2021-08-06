use crate::transcation::Transaction;
use crate::block_hash::BlockHash;

#[derive(Debug, Clone)]
pub struct Transactions(Vec<Transaction>);

impl Transactions {

    pub fn to_hash(&self) -> BlockHash {
        let vec = self.0.iter().map(|v|v.to_hash()).collect::<Vec<BlockHash>>();
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