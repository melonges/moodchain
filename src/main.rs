mod block;
use std::str::FromStr;

use block::Block;

use crate::types::BlockHash;
mod types;
mod utils;
fn main() {
    let genesis_block = Block {
        index: 0,
        prev_hash: BlockHash::from_low_u64_le(0u64),
        data: "GENESIS_BLOCK".to_string(),
        timestamp: 1674044792,
        hash: BlockHash::from_str(
            "17fdd356392f13d5b434aacba8e990388e356336c92b50ece4bc84f3419eedf7",
        )
        .unwrap(),
    };
    let blockchain = vec![genesis_block];
}
