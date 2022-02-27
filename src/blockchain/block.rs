use std::fmt::Display;
use std::time::Instant;

use rand::Rng;
use sha2::{Sha256, Digest};


use super::transaction::{Transaction, self};

static mut ID: u16 = 1;

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u16,
    pub nonce: u32,
    pub prev: String,
    pub difficulty: usize,
    hash: String,
    is_mined: bool,
    pub transaction: Transaction,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            id: unsafe {
                let old = ID;
                ID += 1;
                old
            },
            nonce: rand::thread_rng().gen::<u32>(),
            prev: "0".repeat(64),
            transaction: Default::default(),
            hash: String::new(),
            is_mined: false,
            difficulty: Default::default(),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block[{}]: {}\nwith: {:#?}\nnonce: {}\nprev: {}\n",
            &self.id,
            &self.hash,
            &self.transaction,
            &self.nonce,
            &self.prev,
        )
    }
}

impl Block {
    pub fn new(transaction: Transaction) -> Self {
        let mut block = Self {
            transaction,
            ..Self::default()
        };
        block.hash = block.generate_hash();
        block
    }

    fn generate_hash(&self) -> String {
        let str_block = format!("{self:?}");

        let mut hasher = Sha256::new();
        hasher.update(str_block);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine(&mut self) {
        println!("Staring mining #{} with {} difficulty ...", self.id, self.difficulty);
        let proof = "0".repeat(self.difficulty);

        let start = Instant::now();
        loop {
            if self.hash.starts_with(&proof) {
                break;
            }
            self.nonce += 1;
            self.hash = self.generate_hash();
        }
        self.is_mined = true;
        println!("Took {end:?}", end = start.elapsed());
    }

    pub fn is_mined(&self) -> bool {
        self.is_mined && self.hash.starts_with(&"0".repeat(self.difficulty))
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn difficulty(&self) -> usize {
        self.difficulty
    }
}