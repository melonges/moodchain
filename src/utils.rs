use crate::block::Block;

pub fn is_valid_chain(blockchain_to_validate: &[Block], genesis_block: Block) -> bool {
    if blockchain_to_validate[0] != genesis_block {
        return false;
    }

    for i in 1..blockchain_to_validate.len() {
        if Block::validate_new_block(&blockchain_to_validate[i], &blockchain_to_validate[i - 1]) {
            return false;
        }
    }

    true
}

pub fn replace_chain(
    new_blocks: Vec<Block>,
    genesis_block: Block,
    current_blocks: &mut Vec<Block>,
) {
    if is_valid_chain(&new_blocks, genesis_block) && new_blocks.len() > current_blocks.len() {
        *current_blocks = new_blocks;
    }
}
