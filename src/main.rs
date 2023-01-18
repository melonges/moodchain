mod block;
use block::Block;
fn main() {
    let blockchain = vec![Block {
        index: 0,
        prev_hash: 0x0,
        data: "GENESIS_BLOCK".to_string(),
        timestamp: 1674044792,
        hash: 0x985ed1d54a9f4bdc815eb5d41354ef67,
    }];

    println!("Hello, world!");
}
