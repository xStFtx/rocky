use crate::block::Block;
use log::{info, debug};

pub struct Consensus {
    difficulty: usize,
}

impl Consensus {
    pub fn new(difficulty: usize) -> Self {
        Consensus { difficulty }
    }

    pub fn mine(&self, block: &mut Block) -> Result<(), Box<dyn std::error::Error>> {
        let target = "0".repeat(self.difficulty);
        while !block.calculate_hash_with_nonce(block.nonce)?.starts_with(&target) {
            block.nonce += 1;
            debug!("Nonce: {}, Hash: {}", block.nonce, block.calculate_hash_with_nonce(block.nonce)?);
        }
        block.hash = block.calculate_hash_with_nonce(block.nonce)?;
        info!("Block mined: {}", block.hash);
        Ok(())
    }
}
