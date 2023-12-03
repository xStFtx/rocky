use crate::block::Block;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use log::{info, debug};

pub struct Consensus {
    difficulty: usize,
    mining_interrupt: Arc<AtomicBool>,
}

impl Consensus {
    pub fn new(difficulty: usize) -> Self {
        Consensus { 
            difficulty,
            mining_interrupt: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_difficulty(&mut self, difficulty: usize) {
        self.difficulty = difficulty;
    }

    pub fn interrupt_mining(&self) {
        self.mining_interrupt.store(true, Ordering::SeqCst);
    }

    pub fn mine(&self, block: &mut Block) {
        let target = "0".repeat(self.difficulty);
        let mut nonce = 0;
        
        while !self.mining_interrupt.load(Ordering::SeqCst) {
            let hash = block.calculate_hash_with_nonce(nonce);
            if hash.starts_with(&target) {
                block.nonce = nonce;
                block.hash = hash;
                info!("Block mined: {}", block.hash);
                break;
            }
            nonce += 1;
            debug!("Nonce: {}, Hash: {}", nonce, hash);
        }

        if self.mining_interrupt.load(Ordering::SeqCst) {
            info!("Mining interrupted for block with nonce: {}", nonce);
        }
    }
}
