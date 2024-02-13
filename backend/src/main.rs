mod models;
mod error;
mod utilities;
use models::blockchain::{Blockchain};
use models::block::{Block};
use error::{BlockchainError};
pub type Result<T> = color_eyre::eyre::Result<T, BlockchainError>;
fn main() -> Result<()> {

    let mut ihgedas = Blockchain::new();

    let genesis_block = Block::new_genesis_block();
    let second_block = Block::new("Dummy data 1".to_string(), genesis_block.hash.clone())?;
    let third_block = Block::new("Dummy data 2".to_string(), second_block.hash.clone())?;
    let fourth_block = Block::new("Dummy data 3".to_string(), third_block.hash.clone())?;

    ihgedas.add_block(genesis_block);
    ihgedas.add_block(second_block);
    ihgedas.add_block(third_block);
    ihgedas.add_block(fourth_block);

    ihgedas.print_blockchain();

    Ok(())
}


