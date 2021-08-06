use crate::block_hash::BlockHash;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UserAccountId(u32);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Transaction {
  message: String,
  sender_id: UserAccountId,
  created_at: u64,
  receiver_ids: Vec<UserAccountId>,
}

impl Transaction {
  pub fn to_hash(&self) -> BlockHash {
    let uids = self
      .receiver_ids
      .iter()
      .fold("".to_owned(), |r, e| r + e.0.to_string().as_str());
    let params = vec![
      BlockHash::new(self.message.as_str()),
      self.to_hash(),
      BlockHash::new(&self.created_at.to_string()),
      BlockHash::new(&uids),
    ];
    BlockHash::params(&params)
  }
}
