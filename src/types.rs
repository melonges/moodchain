use std::sync::Mutex;

use crate::block::Block;
pub type BlockHash = String; // h256 needed
pub type UnixTimeStamp = u32;
pub type Blockchain = Vec<Block>;

pub type SharedState = Mutex<Blockchain>;
