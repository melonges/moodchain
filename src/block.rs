use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

type BlockHash = u128;
type UnixTimeStamp = u32;

pub struct Block {
    pub index: u32,
    pub hash: BlockHash,
    pub prev_hash: BlockHash,
    pub timestamp: UnixTimeStamp,
    pub data: String,
}

impl Block {
    pub fn generate_next_block(prev_block: Block, data: String) -> Block {
        let index = prev_block.index + 1;
        let prev_hash = prev_block.hash;
        let timestamp: UnixTimeStamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        Block {
            index,
            hash: Block::hash(index, prev_hash, timestamp, &data),
            prev_hash,
            timestamp,
            data,
        }
    }

    fn hash(
        index: u32,
        prev_hash: BlockHash,
        timestamp: UnixTimeStamp,
        data: &String,
    ) -> BlockHash {
        // TODO make normal hashing
        (prev_hash ^ (timestamp + index) as BlockHash)
    }

    fn validate_new_block(new_block: Block, prev_block: Block) -> bool {
        if prev_block.index + 1 != new_block.index {
            return false;
        } else if prev_block.hash != new_block.prev_hash {
            return false;
        } else if Block::hash(
            new_block.index,
            new_block.hash,
            new_block.timestamp,
            &new_block.data,
        ) != new_block.hash
        {
            return false;
        }
        true
    }
}
