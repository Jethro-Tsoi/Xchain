use crate::block;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain{
    pub fn add_block (&mut self, data: &str) {
        let pre_block = &self.blocks[self.blocks.len() - 1];

        let new_block = block::Block::new(data, &pre_block.hash);
        
        self.blocks.push(new_block);
    }

    pub fn genesis_block () -> block::Block {
        block::Block::new("This is genesis block", "")
    }

    pub fn new_blockchain () -> BlockChain {
         BlockChain { blocks: vec![BlockChain::genesis_block()] }
    }
}