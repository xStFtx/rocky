use crate::transaction::Transaction;
use crate::utils::{calculate_hash, HashInput};
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
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String) -> Result<Block, Box<dyn std::error::Error>> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let nonce = 0;
        let data = json!(&transactions).to_string();

        // Construct HashInput using the public constructor
        let input = HashInput::new(index, timestamp, &data, &previous_hash, nonce);
        let hash = calculate_hash(input)?;

        Ok(Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            nonce,
        })
    }

    pub fn genesis() -> Result<Block, Box<dyn std::error::Error>> {
        Block::new(0, vec![], String::from("0"))
    }

    pub fn calculate_hash_with_nonce(&self, nonce: u64) -> Result<String, Box<dyn std::error::Error>> {
        let data = json!(&self.transactions).to_string();

        // Construct HashInput using the public constructor
        let input = HashInput::new(self.index, self.timestamp, &data, &self.previous_hash, nonce);
        calculate_hash(input)
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
