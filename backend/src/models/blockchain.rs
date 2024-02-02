use super::block::{Block};

#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: Vec<Block>
} impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { blocks: Vec::new() }
    }
    pub fn add_block(&mut self, param_obj: Block) {
        self.blocks.push(param_obj);
    }

    pub fn print_blockchain(&mut self) {
        for block in self.blocks.iter() {
            println!("{block:?}");
        }
    }
}