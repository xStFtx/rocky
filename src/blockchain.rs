use crate::block::Block;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { blocks: vec![Block::genesis()] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn create_block(&self, data: String) -> Block {
        let last_block = self.blocks.last().unwrap();
        let new_index = last_block.index + 1;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Block::new(new_index, timestamp, data, last_block.hash.clone())
    }
}
