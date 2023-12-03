use super::utils::calculate_hash;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64, // Added nonce field
}

impl Block {
    pub fn new(index: u32, timestamp: u64, data: String, previous_hash: String) -> Block {
        let nonce = 0;
        let hash = calculate_hash(index, timestamp, &data, &previous_hash, nonce);
        Block { index, timestamp, data, previous_hash, hash, nonce }
    }

    pub fn genesis() -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let nonce = 0;
        let hash = calculate_hash(0, timestamp, "Genesis Block", "0", nonce);
        Block {
            index: 0,
            timestamp,
            data: String::from("Genesis Block"),
            previous_hash: String::from("0"),
            hash,
            nonce,
        }
    }

    pub fn calculate_hash(&self) -> String {
        calculate_hash(self.index, self.timestamp, &self.data, &self.previous_hash, self.nonce)
    }
}
