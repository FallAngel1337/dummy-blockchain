use std::time::{SystemTime, Duration};
use sha2::{Sha256, Digest};

#[derive(Debug, Default, Clone)]
pub struct Transaction {
    from: Address,
    to: Address,
    amount: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Address {
    name: Option<String>,
    id: String,
}

impl Transaction {
    pub fn new(from: Address, to: Address, amount: usize) -> Self {
        Self {
            from,
            to,
            amount
        }
    }
}

impl Address {
    pub fn new(name: Option<String>) -> Self {
        Self {
            name,
            id: Self::generate_id(),
        }
    }

    fn generate_id() -> String {
        let mut hasher = Sha256::new();
        let now = SystemTime::now();
        std::thread::sleep(Duration::from_millis(500));
        hasher.update(format!("{}", now.elapsed().unwrap().as_millis()));
        format!("{:x}", hasher.finalize())
    }
}