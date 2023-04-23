use anyhow::{Result, Error};
use mockall::automock;

use crate::constants::HASH_COST;

#[automock]
pub trait HashService {
    fn hash(&self, input: &str) -> Result<String>;
    fn verify(&self, raw: &str, hash: &str) -> Result<bool>;
}

#[derive(Debug, Clone)]
pub struct BcryptHashService {
    hash_cost: u32
}

impl BcryptHashService {
    pub fn new() -> Self {
        Self {
            hash_cost: *HASH_COST
        }
    }
}

impl HashService for BcryptHashService {
    fn hash(&self, input: &str) -> Result<String> {
        bcrypt::hash(input, self.hash_cost).map_err(Error::from)
    }

    fn verify(&self, raw: &str, hash: &str) -> Result<bool> {
        bcrypt::verify(raw, hash).map_err(Error::from)
    }
}
