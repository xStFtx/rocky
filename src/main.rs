mod blockchain;
mod block;
mod transaction;
mod utils;
mod consensus;
mod p2p;

use blockchain::Blockchain;
use consensus::Consensus;
use transaction::Transaction;
use p2p::P2P;

fn main() {
    println!("Starting blockchain...");

    let mut blockchain = Blockchain::new();
    let consensus = Consensus::new(2); // Assuming a simple proof of work with difficulty level 2

    // Since Transaction::new now returns a Result, we have to handle the case where it might fail
    match Transaction::new("Alice".to_owned(), "Bob".to_owned(), 50.0) {
        Ok(transaction1) => {
            let block = blockchain.create_block(vec![transaction1]);
            match block {
                Ok(mut block) => {
                    consensus.mine(&mut block);
                    if blockchain.is_valid_new_block(&block) {
                        if let Err(e) = blockchain.add_block(block) {
                            println!("Failed to add block: {}", e);
                        }
                    }
                },
                Err(e) => println!("Failed to create block: {}", e),
            }
        },
        Err(e) => println!("Failed to create transaction: {}", e),
    }

    match Transaction::new("Bob".to_owned(), "Charlie".to_owned(), 30.0) {
        Ok(transaction2) => {
            let block = blockchain.create_block(vec![transaction2]);
            match block {
                Ok(mut block) => {
                    consensus.mine(&mut block);
                    if blockchain.is_valid_new_block(&block) {
                        if let Err(e) = blockchain.add_block(block) {
                            println!("Failed to add block: {}", e);
                        }
                    }
                },
                Err(e) => println!("Failed to create block: {}", e),
            }
        },
        Err(e) => println!("Failed to create transaction: {}", e),
    }

    for block in &blockchain.blocks {
        println!("{:?}", block);
    }

    // The P2P network simulation could go here
    // Assuming you have a function to start the P2P server
    // let p2p_network = P2P::new(8080);
    // p2p_network.start();
}
