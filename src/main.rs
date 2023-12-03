mod blockchain;
mod block;
mod transaction;
mod utils;
mod consensus;
mod p2p;

use blockchain::Blockchain;
use consensus::Consensus;
use transaction::Transaction;

fn main() {
    println!("Starting blockchain...");

    let mut blockchain = Blockchain::new();
    let consensus = Consensus::new(2); // Assuming a simple proof of work with difficulty level 2

    // Create the first transaction and block
    let transaction1 = Transaction::new("Alice".to_owned(), "Bob".to_owned(), 50.0).unwrap();
    let mut block = blockchain.create_block(vec![transaction1]).unwrap(); // Assuming create_block is available in Blockchain
    consensus.mine(&mut block).unwrap();
    blockchain.add_block(block).unwrap();

    // Create the second transaction and block
    let transaction2 = Transaction::new("Bob".to_owned(), "Charlie".to_owned(), 30.0).unwrap();
    let mut block = blockchain.create_block(vec![transaction2]).unwrap(); // Assuming create_block is available in Blockchain
    consensus.mine(&mut block).unwrap();
    blockchain.add_block(block).unwrap();

    // Print all the blocks in the blockchain
    for block in &blockchain.blocks {
        println!("{:?}", block);
    }
}
