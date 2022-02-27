pub mod block;
pub mod transaction;

use std::fmt::Display;

use block::Block;

#[derive(Debug, Clone, Default)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Display for BlockChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.blocks.iter().for_each(|block| println!("{block}"));
        Ok(())
    }
}

impl BlockChain {
    pub fn new(difficulty: usize) -> Self {
        Self {
            difficulty,
            ..Self::default()
        }
    }

    pub fn add(&mut self, mut block: Block) {  
        block.difficulty = self.difficulty;

        if self.blocks.is_empty() {
            self.blocks.push(block);
        } else {
            let last_block = self.blocks.last().unwrap();
            block.prev = last_block.hash().to_owned();
            self.blocks.push(block);
        }
    }

    pub fn mine(&mut self) {
        let mut block_chain = BlockChain::new(self.difficulty);
        self.blocks
            .iter_mut()
            .for_each(|mut block| {
                block.mine();
                block_chain.add(block.clone());
            });
        self.blocks = block_chain.blocks;
    }

    pub fn is_valid(&self) -> bool {
        if !self.blocks.is_empty() && self.blocks.iter().filter(|blk| blk.is_mined()).count() == self.blocks.len() {
            let mut prev_block = self.blocks.first().unwrap();
            
            self.blocks.iter().skip(1).filter(|block| {
                let eq = block.prev == prev_block.hash();
                prev_block = block;
                eq
            })
            .count() + 1 == self.blocks.len()
        } else {
            false
        }
    }
}