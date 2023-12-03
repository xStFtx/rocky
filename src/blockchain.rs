use crate::block::Block;
use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::genesis();
        Blockchain { blocks: vec![genesis_block] }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), &'static str> {
        let last_block = self.blocks.last().expect("Blockchain should have at least one block");
        let new_block = self.create_block(transactions, last_block.index, &last_block.hash)?;

        if self.is_valid_new_block(&new_block, last_block) {
            self.blocks.push(new_block);
            Ok(())
        } else {
            Err("Invalid block")
        }
    }

    fn create_block(&self, transactions: Vec<Transaction>, last_index: u32, last_hash: &str) -> Result<Block, &'static str> {
        let new_index = last_index + 1;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "Time went backwards")?
            .as_secs();

        Ok(Block::new(new_index, transactions, last_hash.to_string()))
    }

    fn is_valid_new_block(&self, new_block: &Block, last_block: &Block) -> bool {
        new_block.index == last_block.index + 1 &&
        new_block.previous_hash == last_block.hash &&
        new_block.hash == new_block.calculate_hash() // Add more validations as needed
    }
}
