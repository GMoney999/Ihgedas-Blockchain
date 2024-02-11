#![allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum BlockError {
    InvalidID,
    InvalidHash,
    InvalidTimestamp,
}

#[derive(Debug, PartialEq)]
pub enum BlockchainError {
    DuplicateBlock,
}

