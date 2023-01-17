use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
// pub type Hash = [u8; 32];
// todo normal export;
pub struct Block {
    pub index: u32,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: String,
    pub data: String,
}

impl Block {
    pub fn generate_next_block(prev_block: Block, data: String) -> Block {
        let index = prev_block.index + 1;
        let prev_hash = prev_block.hash;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        Block {
            index,
            hash: Block::hash(index, &prev_hash, &timestamp, &data),
            prev_hash,
            timestamp,
            data,
        }
    }

    fn hash(index: u32, prev_hash: &String, timestamp: &String, data: &String) -> String {
        let mut hasher = Sha256::new();
        let input = format!("{}{}{}{}", index, prev_hash, timestamp, data);
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
