use crate::types::{BlockHash, UnixTimeStamp};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub hash: BlockHash,
    pub prev_hash: BlockHash,
    pub timestamp: UnixTimeStamp,
    pub data: String,
}

impl Block {
    pub fn generate_next_block(prev_block: &Block, data: String) -> Block {
        let index = prev_block.index + 1;
        let prev_hash = &prev_block.hash;
        let timestamp: UnixTimeStamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        Block {
            index,
            hash: Block::hash(index, prev_hash, timestamp, &data),
            prev_hash: prev_hash.to_string(),
            timestamp,
            data,
        }
    }

    fn hash(index: u32, prev_hash: &BlockHash, timestamp: UnixTimeStamp, data: &str) -> BlockHash {
        let mut hasher = Sha256::new();
        hasher.update(format!("{index}{prev_hash}{timestamp}{data}"));
        format!("{:x}", hasher.finalize())
    }
    pub fn validate_new_block(new_block: &Block, prev_block: &Block) -> Result<(), &'static str> {
        if prev_block.index + 1 != new_block.index {
            return Err("incorrect index");
        } else if prev_block.hash != new_block.prev_hash {
            return Err("incorrect prev hash");
        } else if Block::hash(
            new_block.index,
            &new_block.hash,
            new_block.timestamp,
            &new_block.data,
        ) != new_block.hash
        {
            return Err("incorrect block hash");
        }
        return Ok(());
    }
}
