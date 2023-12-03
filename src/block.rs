use crate::transaction::Transaction;
use crate::utils::calculate_hash;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String) -> Block {
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        let nonce = 0;
        let data = json!(&transactions).to_string();
        let hash = calculate_hash(index, timestamp, &data, &previous_hash, nonce);
        Block { index, timestamp, transactions, previous_hash, hash, nonce }
    }

    pub fn genesis() -> Block {
        Block::new(0, Vec::new(), String::from("0"))
    }

    pub fn calculate_hash_with_nonce(&self, nonce: u64) -> String {
        let data = json!(&self.transactions).to_string();
        calculate_hash(self.index, self.timestamp, &data, &self.previous_hash, nonce)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block {{
    Index: {},
    Timestamp: {},
    Transactions: {:?},
    Previous Hash: {},
    Hash: {},
    Nonce: {}
}}", self.index, self.timestamp, self.transactions, self.previous_hash, self.hash, self.nonce)
    }
}
