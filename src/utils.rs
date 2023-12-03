use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn calculate_hash(index: u32, timestamp: u64, data: &str, previous_hash: &str, nonce: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce));
    format!("{:x}", hasher.finalize())
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
