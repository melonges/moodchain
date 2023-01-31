mod block;
use std::{env, str::FromStr};

use actix_web::{web, App, HttpServer};
use block::Block;
use communication::{get_blockchain, mine};
use primitive_types::H256;
use types::{Blockchain, SharedState};

use crate::types::BlockHash;
mod communication;
mod types;
mod utils;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("cant parse env");
    let addr = env::var("BIND_ADDRESS").expect("expect BIND_ADDRESS");
    let port: u16 = env::var("PORT")
        .expect("expect PORT")
        .parse()
        .unwrap_or(8080);

    let genesis_block = Block {
        index: 0,
        prev_hash: H256::from_low_u64_le(0u64).to_string(),
        data: "GENESIS_BLOCK".to_string(),
        timestamp: 1674044792,
        hash: BlockHash::from_str(
            "17fdd356392f13d5b434aacba8e990388e356336c92b50ece4bc84f3419eedf7",
        )
        .unwrap(),
    };

    let blockchain = web::Data::new(vec![genesis_block]);
    HttpServer::new(move || {
        App::new()
            .app_data(blockchain.clone())
            .service(get_blockchain)
    })
    .bind((addr, port))?
    .run()
    .await
}
