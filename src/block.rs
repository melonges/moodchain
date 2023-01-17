pub type Hash = [u8; 32];
// todo normal export;
pub struct Block {
    pub index: u32,
    pub hash: Hash,
    pub prev_hash: Hash,
    pub timestamp: u32,
    pub data: String,
}

impl Block {
    pub fn generate_next_block(prev_block: Block, data: String) -> Block {
        let index = prev_block.index + 1;
        let prev_hash = prev_block.hash;
        let timestamp = prev_block.timestamp;
        let hash = calculate_hash(index, prev_hash, timestamp, data);
        Block {
            index,
            hash,
            prev_hash,
            timestamp,
            data,
        }
    }

    pub fn hash(&self) -> Hash {}
}
