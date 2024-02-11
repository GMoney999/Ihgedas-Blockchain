mod models;
mod error;
mod utilities;
use models::blockchain::{Blockchain};
use models::block::{Block};

fn main() {

    let mut ihgedas = Blockchain::new();

    let block1 = Block::new("[1] This is the genesis block".to_string());
    let block2 = Block::new("[2] Hash of the genesis block".to_string());
    let block3 = Block::new("[3] Hash of the second block".to_string());
    let block4 = Block::new("[4] Hash of the third block".to_string());

    ihgedas.add_block(block1.unwrap());
    ihgedas.add_block(block2.unwrap());
    ihgedas.add_block(block3.unwrap());
    ihgedas.add_block(block4.unwrap());

    ihgedas.print_blockchain();
}
