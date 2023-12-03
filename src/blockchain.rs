use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::genesis().expect("Failed to create the genesis block");
        Blockchain { blocks: vec![genesis_block] }
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), &'static str> {
        if self.is_valid_new_block(&block) {
            self.blocks.push(block);
            Ok(())
        } else {
            Err("Invalid block cannot be added")
        }
    }

    pub fn create_block(&mut self, transactions: Vec<Transaction>) -> Result<Block, String> {
        let last_block = self.blocks.last().ok_or_else(|| "No last block in the chain".to_string())?;
        let new_index = last_block.index + 1;
        let new_block = Block::new(new_index, transactions, last_block.hash.clone())
            .map_err(|e| e.to_string())?;  // Handling the error correctly
        Ok(new_block)
    }

    fn is_valid_new_block(&self, new_block: &Block) -> bool {
        let last_block = self.blocks.last().expect("Blockchain should have at least one block");
        new_block.index == last_block.index + 1
            && new_block.previous_hash == last_block.hash
            && new_block.calculate_hash_with_nonce(new_block.nonce).expect("Failed to calculate hash") == new_block.hash
    }
}
