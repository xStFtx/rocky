mod blockchain;
mod block;
mod transaction;
mod utils;
mod consensus;
mod p2p;

use blockchain::Blockchain;
use consensus::Consensus;

fn main() {
    println!("Starting blockchain...");

    let mut blockchain = Blockchain::new();
    let consensus = Consensus::new(2); // Assuming a simple proof of work with difficulty level 2

    let mut block = blockchain.create_block("Block 1 Data".to_owned());
    consensus.mine(&mut block);
    blockchain.add_block(block);

    let mut block = blockchain.create_block("Block 2 Data".to_owned());
    consensus.mine(&mut block);
    blockchain.add_block(block);

    for block in &blockchain.blocks {
        println!("{:?}", block);
    }

    // Here you could also add some basic P2P network simulation
}
