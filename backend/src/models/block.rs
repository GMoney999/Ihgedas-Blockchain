#![allow(dead_code)]

use sha2::{Sha256, Digest};
use crate::error::{BlockError};
use crate::utilities::{get_timestamp};
use std::fmt::Write;
pub type Result<T> = color_eyre::eyre::Result<T, BlockError>;

#[derive(Debug, Clone)]
pub struct Block {
    timestamp: u128,
    pub hash: String,
    prev_block_hash: String,
    dummy_data: String,
} impl Block {
    pub fn new_genesis_block() -> Block {
        Block::new(String::from("New genesis block"), String::new()).unwrap()
    }

    pub fn new(dummy_data: String, prev_block_hash: String) -> Result<Block> {
        // Create a timestamp with millisecond precision
        let timestamp = match get_timestamp() {
            Ok(time) => time,
            Err(_) => return Err(BlockError::InvalidTimestamp),
        };

        let mut block = Block {
            timestamp,
            hash: String::new(),
            prev_block_hash,
            dummy_data
        };

        let hash = block.get_hash()?;
        block.hash = hash;
        Ok(block)
    }

    pub fn get_hash(&self) -> Result<String> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.update(&data[..]);
        let result = hasher.finalize();
        let mut hash = String::new();
        for &byte in result.iter() {
            write!(hash, "{:02x}", byte).expect("Writing to a String cannot fail");
        }
        Ok(hash)
    }
    pub fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
                &self.timestamp,
                &self.prev_block_hash,
                &self.dummy_data
            );

        match bincode::serialize(&content) {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(BlockError::ErrorGeneratingHash),
        }

    }
}