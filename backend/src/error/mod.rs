#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum BlockError {
    InvalidID,
    InvalidHash,
    InvalidTimestamp,
    ErrorGeneratingHash,
}

#[derive(Debug, PartialEq)]
pub enum BlockchainError {
    DuplicateBlock,
    ErrorAddingBlock,
    BlockError(BlockError) // Variant to encapsulate block errors
} impl From<BlockError> for BlockchainError {
    fn from(value: BlockError) -> Self {
        BlockchainError::BlockError(value)
    }
}

