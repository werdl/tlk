use serde::{Serialize, Deserialize};

use sha2::Digest;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
    pub timestamp: u128,
    pub user: User
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub public_key: String
}

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> String {
        let bytes = self.bytes();
        let digest = sha2::Sha256::digest(&bytes);
        hex::encode(digest)
    }

    fn verify(&self, proposed_hash: &str) -> bool {
        self.hash() == proposed_hash
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>
}

impl Hashable for User {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.username.as_bytes());
        bytes.extend(self.public_key.as_bytes());
        bytes
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.data.as_bytes());
        bytes.extend(self.prev_hash.as_bytes());
        bytes.extend(self.timestamp.to_string().as_bytes());
        bytes.extend(self.user.bytes());
        bytes
    }
}

pub trait BlockOperations {
    fn new(data: String, prev_hash: String, user: User) -> Self;

    /// chain should not include the block itself
    fn verify_against(&self, chain: &Blockchain) -> bool;
}