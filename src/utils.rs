use sha2::{Sha256, Digest};
use std::fmt::Write;

pub struct HashInput<'a> {
    index: u32,
    timestamp: u64,
    data: &'a str,
    previous_hash: &'a str,
    nonce: u64,
}

impl<'a> HashInput<'a> {
    pub fn new(index: u32, timestamp: u64, data: &'a str, previous_hash: &'a str, nonce: u64) -> HashInput<'a> {
        HashInput {
            index,
            timestamp,
            data,
            previous_hash,
            nonce,
        }
    }
}

pub fn calculate_hash(input: HashInput) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}{}{}", input.index, input.timestamp, input.data, input.previous_hash, input.nonce));

    let mut hash_hex = String::new();
    for byte in hasher.finalize().iter() {
        write!(&mut hash_hex, "{:02x}", byte)?;
    }

    Ok(hash_hex)
}
