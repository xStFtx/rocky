use crate::block::Block;

pub struct Consensus {
    difficulty: usize,
}

impl Consensus {
    pub fn new(difficulty: usize) -> Self {
        Consensus { difficulty }
    }

    pub fn mine(&self, block: &mut Block) {
        while !block.hash.starts_with(&"0".repeat(self.difficulty)) {
            block.nonce += 1;
            block.hash = block.calculate_hash();
        }
    }
}
