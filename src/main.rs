#![allow(unused)]

mod blockchain;
use std::ops::Add;

use blockchain::{
    BlockChain,
    block::Block,
    transaction::*,
};

fn main() {
    let mut block_chain = BlockChain::new(3);
    block_chain.add(Block::new(
        Transaction::new(Address::new(Some("John".to_owned())), Address::new(Some("Ann".to_owned())), 100)
    ));
    block_chain.mine();

    let mut block_chain2 = block_chain.clone();
    block_chain2.add(Block::new(
        Transaction::new(Address::new(None), Address::new(None), 100)
    ));
    block_chain2.mine();
    
    /// NOTE: `block` field from `BlockChain` structure would be tagged as `priv`.
    // let mut block = Block::new(
    //     Transaction::new(Address::new(Some("AAAAA".to_owned())), Address::new(None), 100)
    // );
    // block.difficulty = 3;
    // block_chain2.blocks[0] = block;
    // block_chain2.mine();
    
    println!("{block_chain2}");

    println!("IS VALID? {}", block_chain2.is_valid());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut block_chain = BlockChain::new(3);

        let mut block = Block::new(
            Transaction::new(Address::new(Some("John".to_owned())), Address::new(Some("Ann".to_owned())), 100)
        );
        
        let mut block_with_data = Block::new(
            Transaction::new(Address::new(None), Address::new(None), 100)
        );

        block_chain.add(block);
        block_chain.add(block_with_data);
        block_chain.mine();
    }
}