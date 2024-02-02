
#[derive(Debug, Clone)]
pub struct Block {
    timestamp: u128,
    hash: String,
    prev_block_hash: String,
} impl Block {
    pub fn new(prev_block_hash: String) -> Block {
        let timestamp: u128 = 0;
        let hash = "Current block hash here".to_string();

        Block {
            timestamp,
            hash,
            prev_block_hash
        }
    }
}