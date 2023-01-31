use actix_web::web;

use crate::block::Block;
use crate::types::{Blockchain, SharedState};
pub fn is_valid_chain(blockchain_to_validate: &[Block], genesis_block: Block) -> bool {
    if blockchain_to_validate[0] != genesis_block {
        return false;
    }
    for i in 1..blockchain_to_validate.len() {
        if let Err(_) =
            Block::validate_new_block(&blockchain_to_validate[i], &blockchain_to_validate[i - 1])
        {
            return false;
        }
    }
    true
}

pub fn replace_chain(
    new_blocks: Blockchain,
    genesis_block: Block,
    current_blocks: &mut Blockchain,
) {
    if is_valid_chain(&new_blocks, genesis_block) && new_blocks.len() > current_blocks.len() {
        *current_blocks = new_blocks;
    }
}
