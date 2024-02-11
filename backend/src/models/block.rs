#![allow(dead_code)]
use crate::error::{BlockError};
use crate::utilities::{get_timestamp};
pub type Result<T> = color_eyre::eyre::Result<T, BlockError>;

#[derive(Debug, Clone)]
pub struct Block {
    timestamp: u128,
    hash: String,
    prev_block_hash: String,
} impl Block {
    pub fn new(prev_block_hash: String) -> Result<Block> {
        let hash = "Current block hash here".to_string();

        // Create a timestamp with millisecond precision
        let timestamp = match get_timestamp() {
            Ok(time) => time,
            Err(_) => return Err(BlockError::InvalidTimestamp),
        };

        Ok(Block {
            timestamp,
            hash,
            prev_block_hash
        })
    }

}